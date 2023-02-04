//! This module represents the [`proxy's communicator`](Communicator). \
//! It accepts new connections from the MCManage network and manages the sending and receiving of [`messages`](mcm_misc::message::Message).\
//! \
//! ```text
//! -------------------------------------
//! |           Communicator            |
//! |                                   |
//! |    ----------------------------   |
//! |    | Communicator main thread |   |   Creates a handler for each new connection.
//! |    ----------------------------   |
//! |         |                |        |
//! |         |                |        |
//! |         V                V        |
//! |    -----------      -----------   |   Send and receive messages to and from their 
//! |    | Handler |      | Handler |   |   connected client.
//! |    -----------      -----------   |   A client could be a Runner or a Client application.
//! |      Λ                      Λ     |
//! |      |                      |     |   Send and receive channels to transmit messages.
//! |      |                      |     |
//! |      |     ------------     |     |   Passes on received messages to the right receiver.
//! |      ----> | InterCom | <----     |   This can be a handler or the console.
//! |            ------------           |   
//! |                 Λ                 |
//! |                 |                 |
//! ----------------- | -----------------   Send and receive channels to transmit messages.
//!                   |
//!                   V
//! -------------------------------------   Here, all received commands get executed and
//! |              Console              |   messages for clients are created.
//! -------------------------------------
//! ```


use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream, Shutdown, SocketAddr};
use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::mpsc::{self, Sender, Receiver, TryRecvError};
use std::{thread, io};
use std::time::{Duration, Instant};

use mcm_misc::concurrent_class::ConcurrentClass;
use mcm_misc::log;
use mcm_misc::mcmanage_error::MCManageError;
use mcm_misc::message::Message;
use mcm_misc::message::message_type::MessageType;
use mcm_misc::config_trait::ConfigTrait;

use self::intercom::InterCom;
use communicator_error::CommunicatorError;


mod tests;
mod intercom;
pub mod communicator_error;

/// This struct manages the communication between this application and other ones connected to it via a socket connection. In this case, there are two kinds of connected clients:
/// the [`Runner`](https://github.com/Gooxey/mcm_runner.git) or the [`Client`](https://github.com/Gooxey/mcm_runner.git). For every new client, a new [`handler`](super::Communicator::handler)
/// gets started, which is responsible for sending [`messages`](mcm_misc::message::Message) received from the [`InterCom`] to the connected client and [`messages`](mcm_misc::message::Message)
/// received from the connected client to the [`InterCom`].
/// 
/// ## Methods
/// 
/// | Method                                                 | Description                                                                                                                             |
/// |--------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------|
/// | [`start(...) -> Result<...>`](Communicator::start)     | Start the [`Communicator`] and its [`InterCom`]. The instance of the [`Communicator`] will then be returned inside an Arc-Mutex bundle. |
/// | [`stop(...)`](Communicator::stop)                      | Stop the [`Communicator`] and its [`InterCom`].                                                                                         |
/// | [`self_stop(...)`](Communicator::self_stop)            | This method gets used by threads wanting to stop the [`Communicator`] and its [`InterCom`] because of a fatal error.                    |
/// | [`restart(...) -> Result<...>`](Communicator::restart) | Restart the [`Communicator`] and its [`InterCom`].                                                                                      |
/// | [`self_restart(...)`](Communicator::self_restart)      | This method gets used by threads wanting to restart the [`Communicator`] and its [`InterCom`] because of a fatal error.                 |
pub struct Communicator<C: ConfigTrait> {
    /// This application's config.
    config: Arc<C>,
    /// This Communicator's InterCom.
    intercom: Arc<Mutex<InterCom<C>>>,
    /// This Communicator's main thread.
    main_thread: Option<thread::JoinHandle<()>>,
    /// Controls whether or not the [`main thread`](Communicator::main) and the [`handlers`](Communicator::handler) are active.
    alive: bool
}
impl<C: ConfigTrait> ConcurrentClass<Communicator<C>, C> for Communicator<C> {
    fn get_config_unlocked(class_lock: &MutexGuard<Communicator<C>>) -> Arc<C> {
        class_lock.config.clone()
    }
    fn get_name_unlocked(_: &MutexGuard<Communicator<C>>) -> String {
        "Communicator".to_string()
    }
    fn get_name_poison_error(_: &MutexGuard<Communicator<C>>) -> String {
        "Communicator".to_string()
    }
    fn get_default_state(class_lock: &mut MutexGuard<Communicator<C>>) -> Communicator<C> {
        Communicator {
            config: class_lock.config.clone(),
            intercom: class_lock.intercom.clone(),
            main_thread: None,
            alive: false
        }
    }
    fn start(class: &Arc<Mutex<Communicator<C>>>, log_messages: bool) -> Result<(), MCManageError> {
        let mut class_lock;
        if let Some(lock) = Self::get_lock_pure(class, false) {
            class_lock = lock;
        } else {
            if log_messages { log!("erro", "Communicator", "This Communicator got corrupted."); }
            Self::reset(&class);
            return Err(MCManageError::FatalError);
        }

        let (bootup_status_send, bootup_status_receive) = mpsc::channel::<bool>();

        // declare the Communicator to be active
        class_lock.alive = true;
        
        // start the main thread
        let class_clone = class.clone();
        class_lock.main_thread = Some(thread::spawn(move || {
            if let Err(_) = Self::main(&class_clone, bootup_status_send, true) {}
        }));

        let intercom = class_lock.intercom.clone();
        drop(class_lock);
        // wait for the bootup status of the main thread
        // true  -> bootup was successful
        // false -> bootup failed
        if let Ok(bootup_status) = bootup_status_receive.recv() {
            if bootup_status == false {
                // the error messages gets printed by the main thread
                Self::reset(&class);
                return Err(MCManageError::FatalError);
            }
        } else {
            log!("erro", "Communicator", "The main thread crashed. The Communicator could not be started.");
            Self::reset(&class);
            return Err(MCManageError::FatalError);
        }
        
        // start the InterCom
        InterCom::set_communicator(&intercom, &class);
        if let Err(erro) = InterCom::start(&intercom, true){
            Self::reset(&class);
            return Err(erro);
        }

        Ok(())
    }
    fn stop(class: &Arc<Mutex<Communicator<C>>>, log_messages: bool) -> Result<(), MCManageError> {
        let mut class_lock;
        if let Some(lock) = Self::get_lock_pure(class, false) {
            class_lock = lock;
        } else {
            if log_messages { log!("erro", "Communicator", "This Communicator got corrupted."); }
            Self::reset(&class);
            return Err(MCManageError::FatalError);
        }
        
        let stop_time = Instant::now();
        log!("", "Communicator", "Shutting down...");


        // stop the InterCom
        InterCom::stop(&class_lock.intercom, true)?;
    
        // give the shutdown command
        class_lock.alive = false;

        // wait for all threads to finish
        if let Some(main_thread) = class_lock.main_thread.take() {
            drop(class_lock);
            main_thread.join().expect("Could not join spawned thread");
        }

        log!("", "Communicator", "Stopped in {:.3} secs!", stop_time.elapsed().as_secs_f64()); 

        Ok(())
    }
}
impl<C: ConfigTrait> Communicator<C> {
    /// Create a new [`Communicator`] instance.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                     | Description                                                                                                                                      |
    /// |-------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------|
    /// | `config: Arc<Config>>`        | This application's config.                                                                                                                       |
    /// | `sender: Sender<Message>`     | This channel will be used by the [`InterCom`] to pass on [`messages`](mcm_misc::message::Message) to the [`console`](crate::console::Console).   |
    /// | `receiver: Receiver<Message>` | This channel will be used by the [`InterCom`] to receive [`messages`](mcm_misc::message::Message) from the [`console`](crate::console::Console). |
    pub fn new(config: Arc<C>, sender: Sender<Message>, receiver: Receiver<Message>) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            config: config.clone(),
            intercom: InterCom::new(config.clone(), sender, receiver),
            main_thread: None,
            alive:false
        }))
    }

    /// This method gets used by threads wanting to stop the [`Communicator`] and its [`InterCom`] because of a fatal error.
    pub fn self_stop(communicator: &Arc<Mutex<Communicator<C>>>) {
        let com = communicator.clone();
        thread::spawn(move || 
            Self::stop(&com, true)
        );
    }

    /// Get the current alive status of a given Communicator. \
    /// \
    /// Note: A value of false can be the true value or an indicator of a corrupted Communicator.
    fn get_alive(communicator: &Arc<Mutex<Communicator<C>>>) -> Result<bool, MCManageError> {
        if let Ok(communicator_lock) = Self::get_lock_nonblocking(&communicator) {
            return Ok(communicator_lock.alive.clone());
        } else {
            log!("", "Communicator", "The Communicator got corrupted. It will be restarted.");
            Self::self_restart(communicator);
            return Err(MCManageError::CriticalError);
        }
    }
    /// Get the [`InterCom struct`](crate::config::Config) used by a given Communicator.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                                 | Description                |
    /// |-------------------------------------------|----------------------------|
    /// | `communicator: &Arc<Mutex<Communicator>>` | The Communicator to check. |
    fn get_config(communicator: &Arc<Mutex<Communicator<C>>>) -> Result<Arc<C>, MCManageError> {
        if let Ok(communicator_lock) = Self::get_lock_nonblocking(&communicator) {
            return Ok(communicator_lock.config.clone());
        } else {
            log!("", "Communicator", "The Communicator got corrupted. It will be restarted.");
            Self::self_restart(communicator);
            return Err(MCManageError::CriticalError);
        }
    }
    /// Get the [`InterCom struct`](InterCom) used by a given Communicator.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                                 | Description                |
    /// |-------------------------------------------|----------------------------|
    /// | `communicator: &Arc<Mutex<Communicator>>` | The Communicator to check. |
    fn get_intercom(communicator: &Arc<Mutex<Communicator<C>>>) -> Result<Arc<Mutex<InterCom<C>>>, MCManageError> {
        if let Ok(communicator_lock) = Self::get_lock_nonblocking(&communicator) {
            return Ok(communicator_lock.intercom.clone());
        } else {
            log!("", "Communicator", "The Communicator got corrupted. It will be restarted.");
            Self::self_restart(communicator);
            return Err(MCManageError::CriticalError);
        }
    }

    /// This function represents the main loop of the [`Communicator`] and is intended to be run in a thread. \
    /// It will constantly check for new clients wanting to connect. If it detects a new client, a new [`handler thread`](Communicator::handler) will be started to handle
    /// the [`messages`](mcm_misc::message::Message) sent between the client and [`InterCom`].
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                                 | Description                                                                                                 |
    /// |-------------------------------------------|-------------------------------------------------------------------------------------------------------------|
    /// | `communicator: &Arc<Mutex<Communicator>>` | The [`Communicator`] which started this function.                                                           |
    /// | `bootup_status: Sender<bool>`             | A channel used to inform the [`start method`](Communicator::start) of the success or failure of the bootup. |
    /// | `first_start: bool`                       | Informs this function wether or not this is a start or restart.                                             |
    fn main(communicator: &Arc<Mutex<Communicator<C>>>, bootup_status: Sender<bool>, first_start: bool) -> Result<(), MCManageError> {
        let mut handlers: Vec<thread::JoinHandle<()>> = vec![];

        let start_time = Instant::now();
        if first_start {
            log!("info", "Communicator", "Starting...");
        }

        let mut tries = 0;
        while Self::get_alive(&communicator)? {
            tries += 1;

            match TcpListener::bind(Self::get_config(&communicator)?.addr()) {
                Ok(tcplistener) => {
                    if let Err(err) = tcplistener.set_nonblocking(true) {
                        log!("erro", "Communicator", "Failed to activate `non-blocking mode` for the socket server! The Communicator will be restarted. Error: {err}");
                        log!("erro", "Communicator", "The Communicator will be restarted.");
                        
                        Self::self_restart(communicator);
                        return Err(MCManageError::CriticalError);
                    }

                    if first_start {
                        log!("info", "Communicator", "Started in {:.3} secs!", start_time.elapsed().as_secs_f64());
                    }

                    // the TCPListener got started -> inform the start method of the successful bootup
                    if let Err(_) = bootup_status.send(true) {
                        log!("erro", "Communicator", "The thread starting the Communicator got stopped!");
                        log!("erro", "Communicator", "The Communicator will now shut down.");

                        Self::get_lock_nonblocking(communicator)?.alive = false;
                        return Err(MCManageError::FatalError);
                    }

                    // the main loop of the tcplistener
                    while Self::get_alive(&communicator)? {
                        match tcplistener.accept() {
                            Ok(client) => {
                                // create a new thread for the client
                                let communicator_clone = communicator.clone();
                                handlers.push(thread::spawn(move || {
                                    if let Err(_) = Self::handler(client.0, client.1, &communicator_clone) {}
                                }));
                            }
                            Err(erro) if erro.kind() == io::ErrorKind::WouldBlock => { /* There was no client to be accepted -> ignore this */ }
                            Err(erro) => {
                                log!("warn", "Communicator", "Found an error while accepting new clients. Error: {erro}");
                                /* It is now the clients responsibility to retry the connection */
                            }
                        }
                        thread::sleep(*Self::get_config(&communicator)?.refresh_rate());
                    }
                }
                Err(err) => {
                    if tries == *Self::get_config(&communicator)?.max_tries() {
                        // the TCPListener failed to start -> inform the start method of the failed bootup
                        if let Err(_) = bootup_status.send(false) {
                            log!("erro", "Communicator", "The Communicator failed to start.");
                            log!("erro", "Communicator", "The thread starting the Communicator got stopped.");

                            return Err(MCManageError::FatalError);
                        }

                        log!("erro", "Communicator", "The maximum number of tries has been reached.");
                        return Err(MCManageError::FatalError);
                    }
                    else {
                        log!("warn", "Communicator", "Received an error when trying to bind the socket server. Error: {err}");
                        log!("warn", "Communicator", "This was try number {tries}. 3 seconds till the next one.");
                        thread::sleep(Duration::new(3, 0));
                    } 
                }
            }
        }

        // The Communicator got stopped -> Wait for all handlers to finish before stopping too
        for handler in handlers {
            handler.join().expect("Could not join on stopping handler thread!")
        }
        Ok(())
    }

    /// This function represents the main loop of the handler and is intended to be run in a thread. \
    /// It will constantly check and redirect [`messages`](mcm_misc::message::Message) received by the [`InterCom`] to the connected client, and
    /// [`messages`](mcm_misc::message::Message) received by the connected client will be redirected to the [`InterCom`].
    /// 
    ///  ## Parameters
    /// 
    /// | Parameter                                 | Description                                      |
    /// |-------------------------------------------|--------------------------------------------------|
    /// | `mut client: TcpStream`                   | The client to communicate with.                  |
    /// | `ip: SocketAddr`                          | The clients ip.                                  |
    /// | `communicator: &Arc<Mutex<Communicator>>` | The [`Communicator`] which started this handler. |
    fn handler(mut client: TcpStream, ip: SocketAddr, communicator: &Arc<Mutex<Communicator<C>>>) -> Result<(), CommunicatorError> {
        let id: String;
        let intercom_sender: Sender<Message>;
        let intercom_receiver: Receiver<Message>;
        let mut buffer = vec![0; *Self::get_config(&communicator)?.buffsize() as usize];
        
        log!("info", "Communicator", "A new client has connected using the IP address `{}`.", ip);

        // Register the client at the InterCom
        match Self::register_client(&mut client, ip, Self::get_intercom(&communicator)?, &Self::get_config(&communicator)?) {
            Ok(result) => {
                (id, intercom_sender, intercom_receiver) = result;
            }
            Err(erro) => {
                match erro {
                    CommunicatorError::ConnectionError => {
                        return Self::close_connection_ip(&mut client, &ip);
                    }
                    _ => {
                        if let Err(_) = Self::close_connection_ip(&mut client, &ip) {}
                        Self::self_restart(communicator);

                        return Err(CommunicatorError::MCManageError(MCManageError::CriticalError));
                    }
                }
            }
        }
        
        // activate the nonblocking mode
        if let Err(err) = client.set_nonblocking(true) {
            log!("erro", "Communicator", "Failed to activate the `nonblocking` mode for the client {id}. This Connection will be closed. Error: {err}");
            return Self::close_connection_id(&mut client, &id);
        }

        // The main loop of the handler
        while Self::get_alive(&communicator)? {
            // pass on messages from the InterCom to the client
            match intercom_receiver.try_recv() {
                Ok(msg) => {
                    match client.write(
                        match &msg.to_bytes() {
                            Some(bytes_str) => { bytes_str }
                            None => {
                                log!("erro", "Communicator", "Failed to convert the received bytes-string from {id} to a Message. This connection will be closed.");
                                return Self::close_connection_id(&mut client, &id);
                            }
                        }
                    ) {
                        Ok(n) => {
                            if n == 0 {
                                log!("info", "Communicator", "The client {id} disconnected.");
                                return Self::close_connection_id(&mut client, &id);
                            }
                        }
                        Err(err) => {
                            log!("erro", "Communicator", "An error occurred while writing to a message to the client {id}. This connection will be closed. Error: {err}");
                            return Self::close_connection_id(&mut client, &id);
                        }
                    }
                }
                Err(err) if err == TryRecvError::Empty => { /* There was no message from the InterCom -> ignore this */ }
                Err(_) => {
                    log!("erro", "Communicator", "The connection to the InterCom got interrupted. The Communicator will be restarted.");
                    if let Err(_) = Self::close_connection_id(&mut client, &id) {}
                    Self::self_restart(communicator);

                    return Err(CommunicatorError::MCManageError(MCManageError::CriticalError));
                }
            }

            // pass on messages from the client
            match client.read(&mut buffer) {
                Ok(n) => {
                    if n == 0 {
                        log!("info", "Communicator", "The client {id} disconnected.");
                        return Self::close_connection_id(&mut client, &id);
                    }

                    let msg: Message;
                    // create a message from the received bytes-string
                    if let Some(result) = Message::from_bytes(buffer.to_vec()) {
                        msg = result;
                    } else {
                        log!("erro", "Communicator", "Failed to convert the received bytes-string from {id} to a Message. This connection will be closed.");
                        return Self::close_connection_id(&mut client, &id);
                    }
                    // send this message to the InterCom
                    if let Err(err) = intercom_sender.send(msg) {
                        log!("erro", "Communicator", "An error occurred while writing a message from the client {id} to the InterCom. This connection will be closed. Error: {err}");
                        return Self::close_connection_id(&mut client, &id);
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => { /* The client did not sent anything -> Do nothing */ }
                Err(err) => {
                    log!("erro", "Communicator", "An error occurred while reading a message from the client {id}. This connection will be closed. Error: {err}");
                    return Self::close_connection_id(&mut client, &id);
                }
            }

            thread::sleep(*Self::get_config(&communicator)?.refresh_rate());
        }

        Ok(())
    }
    /// This function will register a given client at the [`InterCom`].
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                        | Description                                          |
    /// |----------------------------------|------------------------------------------------------|
    /// | `client: &mut TcpStream`         | The client to communicate with.                      |
    /// | `ip: SocketAddr`                 | The clients ip.                                      |
    /// | `intercom: Arc<Mutex<InterCom>>` | The [`Communicator's`](Communicator) [`InterCom`].   |
    /// | `config: &Arc<Config>`           | The application's [`config`](crate::config::Config). |
    fn register_client(client: &mut TcpStream, ip: SocketAddr, intercom: Arc<Mutex<InterCom<C>>>, config: &Arc<C>) -> Result<(String, Sender<Message>, Receiver<Message>), CommunicatorError> {
        let id: String;
        let intercom_sender: Sender<Message>;
        let intercom_receiver: Receiver<Message>;
        
        // deactivate the nonblocking mode
        if let Err(err) = client.set_nonblocking(false) {
            log!("erro", "Communicator", "Failed to deactivate the `nonblocking` mode for the client {ip}. This Connection will be closed. Error: {err}");
            return Err(CommunicatorError::ConnectionError);
        }
        
        // get the client type (runner or client)
        let client_type = Self::register_client_get_type(client, &ip, config)?;
        
        // register at the InterCom as a handler
        match InterCom::add_handler(&intercom, client_type) {
            Ok(result) => { (id, intercom_sender, intercom_receiver) = result; }
            Err(err) => {
                log!("erro", "Communicator", "Failed to register the client {ip} as handler at the InterCom! This Connection will be closed. Error: {err}");
                return Err(CommunicatorError::ConnectionError);
            }
        }
        log!("", "Communicator", "The client {ip} has been registered as {id}.");

        // inform the client about the end of this registration process
        match client.write(&vec![0]) {
            Ok(n) => {
                if n == 0 {
                    log!("info", "Communicator", "The client {id} disconnected.");
                    return Err(CommunicatorError::ConnectionError);
                }
            }
            Err(err) => {
                log!("erro", "Communicator", "An error occurred while writing to a message to the client {id}. This connection will be closed. Error: {err}");
                return Err(CommunicatorError::ConnectionError);
            }
        }

        Ok((id, intercom_sender, intercom_receiver))
    }
    /// This function will communicate with a given client to find out its type. There are three outcomes: \
    /// the client is a [`Runner`](https://github.com/Gooxey/mcm_runner.git); the client is a [`Client`](https://github.com/Gooxey/mcm_client.git); the client is invalid,
    /// and the connection gets closed.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                | Description                                          |
    /// |--------------------------|------------------------------------------------------|
    /// | `client: &mut TcpStream` | The client to communicate with.                      |
    /// | `ip: &SocketAddr`        | The clients ip.                                      |
    /// | `config: &Arc<Config>`   | The application's [`config`](crate::config::Config). |
    fn register_client_get_type(client: &mut TcpStream, ip: &SocketAddr, config: &Arc<C>) -> Result<char, CommunicatorError>{
        let mut buffer = vec![0; *config.buffsize() as usize];
        let client_type: char;
        
        match client.write(            
            match &Message::new("get_client_type", MessageType::Request, "communicator", "", vec![]).to_bytes() {
                Some(bytes_str) => { bytes_str }
                None => {
                    log!("erro", "Communicator", "Failed to convert the received bytes-string from {ip} to a Message. This connection will be closed.");
                    return Err(CommunicatorError::ConnectionError);
                }
            }
        ) {
            Ok(n) => {
                if n == 0 {
                    log!("", "Communicator", "The client {ip} disconnected.");
                    return Err(CommunicatorError::ConnectionError);
                }
            }
            Err(err) => {
                log!("erro", "Communicator", "An error occurred while writing to a message to the client {ip}. This connection will be closed. Error: {err}");
                return Err(CommunicatorError::ConnectionError);
            }
        }
        match client.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    log!("", "Communicator", "The client {ip} disconnected.");
                    return Err(CommunicatorError::ConnectionError);
                }
                
                let msg: Message;
                if let Some(m) = Message::from_bytes(buffer.to_vec()) {
                    msg = m;
                } else {
                    log!("erro", "Communicator", "Failed to convert the received bytes-string from {ip} to a Message. This connection will be closed.");
                    return Err(CommunicatorError::ConnectionError);
                }

                match msg.message_type() {
                    MessageType::Response => { /* This should happen */ }
                    _ => {
                        log!("erro", "Communicator", "Expected the first message from {ip} to be an response. This connection will be closed.");
                        return Err(CommunicatorError::ConnectionError);
                    }
                }
                
                if msg.command() == "get_client_type" {
                    if let Some(char) = msg.args()[0].clone().chars().next() {
                        client_type = match char {
                            'r' => char,
                            'c' => char,
                            _ => {
                                log!("erro", "Communicator", "Received an invalid client_type from the client {ip}. This connection will be closed.");
                                return Err(CommunicatorError::ConnectionError);
                            }
                        }
                    } else {
                        log!("erro", "Communicator", "Received an empty client_type from the client {ip}. This connection will be closed.");
                        return Err(CommunicatorError::ConnectionError);
                    }
                }
                else {
                    log!("erro", "Communicator", "Received an invalid first message from the client {ip}. This connection will be closed.");
                    return Err(CommunicatorError::ConnectionError);
                }

            }
            Err(err) => {
                log!("erro", "Communicator", "An error occurred while reading a message from the client {ip}. This connection will be closed. Error: {err}");
                return Err(CommunicatorError::ConnectionError);
            }
        }

        Ok(client_type)
    }

    /// Close the socket connection given. \
    /// If the shutdown command fails, an error message gets printed.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                | Description              |
    /// |--------------------------|--------------------------|
    /// | `client: &mut TcpStream` | The connection to close. |
    /// | `ip: &SocketAddr`        | The clients ip.          |
    fn close_connection_ip(client: &mut TcpStream, ip: &SocketAddr) -> Result<(), CommunicatorError> {
        if let Err(err) = client.shutdown(Shutdown::Both) {
            log!("erro", "Communicator", "An error occurred when trying to close the connection to the client {ip}. Error: {err}");
        }
        return Err(CommunicatorError::ConnectionError);
    }
    /// Close the socket connection given. \
    /// If the shutdown command fails, an error message gets printed.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                | Description              |
    /// |--------------------------|--------------------------|
    /// | `client: &mut TcpStream` | The connection to close. |
    /// | `ip: &SocketAddr`        | The clients id.          |
    fn close_connection_id(client: &mut TcpStream, id: &String) -> Result<(), CommunicatorError> {
        if let Err(err) = client.shutdown(Shutdown::Both) {
            log!("erro", "Communicator", "An error occurred when trying to close the connection to the client {id}. Error: {err}");
        }
        return Err(CommunicatorError::ConnectionError);
    }
}