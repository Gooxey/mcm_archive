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
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Sender, Receiver, TryRecvError};
use std::{thread, io};
use std::time::{Duration, Instant};

use mcm_misc::log::log;
use mcm_misc::message::Message;

use crate::config::Config;
use self::intercom::InterCom;
use crate::error::CommunicatorError;


mod tests;
mod intercom;

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
pub struct Communicator {
    /// This application's config.
    config: Arc<Config>,
    /// This Communicator's InterCom.
    intercom: Arc<Mutex<InterCom>>,
    /// This Communicator's main thread.
    main_thread: Option<thread::JoinHandle<()>>,
    /// Controls whether or not the [`main thread`](Communicator::main) and the [`handlers`](Communicator::handler) are active.
    alive: Arc<AtomicBool>
}
impl Communicator {
    /// Create a new [`Communicator`] instance.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                     | Description                                                                                                                                      |
    /// |-------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------|
    /// | `config: Arc<Config>>`        | This application's config.                                                                                                                       |
    /// | `sender: Sender<Message>`     | This channel will be used by the [`InterCom`] to pass on [`messages`](mcm_misc::message::Message) to the [`console`](crate::console::Console).   |
    /// | `receiver: Receiver<Message>` | This channel will be used by the [`InterCom`] to receive [`messages`](mcm_misc::message::Message) from the [`console`](crate::console::Console). |
    fn new(config: Arc<Config>, sender: Sender<Message>, receiver: Receiver<Message>) -> Self {
        Self {
            config: config.clone(),
            intercom: Arc::new(Mutex::new(InterCom::new(config.clone(), sender, receiver))),
            main_thread: None,
            alive: Arc::new(AtomicBool::new(false))
        }
    }

    /// Get the current alive status of a given Communicator.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                                 | Description                |
    /// |-------------------------------------------|----------------------------|
    /// | `communicator: &Arc<Mutex<Communicator>>` | The Communicator to check. |
    fn get_alive(communicator: &Arc<Mutex<Communicator>>) -> bool {
        let alive: bool;
        if let Ok(communicator) = communicator.lock() {
            alive = communicator.alive.load(Ordering::Relaxed);
        } else {
            log("erro", "Communicator", "The Communicator got corrupted.");
            unimplemented!("handle Communicator corrupted");
        }
        return alive;
    }
    /// Set the current alive status of a given Communicator.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                                 | Description               |
    /// |-------------------------------------------|---------------------------|
    /// | `communicator: &Arc<Mutex<Communicator>>` | The Communicator to edit. |
    fn set_alive(communicator: &Arc<Mutex<Communicator>>, value: bool) {
        if let Ok(communicator) = communicator.lock() {
            communicator.alive.store(value, Ordering::Relaxed);
        } else {
            log("erro", "Communicator", "The Communicator got corrupted.");
            unimplemented!("handle Communicator corrupted");
        }
    }
    /// Get the [`InterCom struct`](crate::config::Config) used by a given Communicator.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                                 | Description                |
    /// |-------------------------------------------|----------------------------|
    /// | `communicator: &Arc<Mutex<Communicator>>` | The Communicator to check. |
    fn get_config(communicator: &Arc<Mutex<Communicator>>) -> Arc<Config> {
        let config: Arc<Config>;
        if let Ok(communicator) = communicator.lock() {
            config = communicator.config.clone()
        } else {
            log("erro", "Communicator", "The Communicator got corrupted.");
            unimplemented!("handle Communicator corrupted");
        }
        return config;
    }
    /// Get the [`InterCom struct`](InterCom) used by a given Communicator.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                                 | Description                |
    /// |-------------------------------------------|----------------------------|
    /// | `communicator: &Arc<Mutex<Communicator>>` | The Communicator to check. |
    fn get_intercom(communicator: &Arc<Mutex<Communicator>>) -> Arc<Mutex<InterCom>> {
        let intercom: Arc<Mutex<InterCom>>;
        if let Ok(communicator) = communicator.lock() {
            intercom = communicator.intercom.clone()
        } else {
            log("erro", "Communicator", "The Communicator got corrupted.");
            unimplemented!("handle Communicator corrupted");
        }
        return intercom;
    }

    /// Start the [`Communicator`] and its [`InterCom`]. The instance of the [`Communicator`] will then be returned inside an Arc-Mutex bundle.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                     | Description                                                                                                                                      |
    /// |-------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------|
    /// | `config: Arc<Config>>`        | This application's config.                                                                                                                       |
    /// | `sender: Sender<Message>`     | This channel will be used by the [`InterCom`] to pass on [`messages`](mcm_misc::message::Message) to the [`console`](crate::console::Console).   |
    /// | `receiver: Receiver<Message>` | This channel will be used by the [`InterCom`] to receive [`messages`](mcm_misc::message::Message) from the [`console`](crate::console::Console). |
    pub fn start(config: Arc<Config>, sender: Sender<Message>, receiver: Receiver<Message>) -> Result<Arc<Mutex<Self>>, CommunicatorError> {        
        let communicator = Arc::new(Mutex::new(Communicator::new(config, sender, receiver)));
        let (bootup_status_send, bootup_status_receive) = mpsc::channel::<bool>();


        // declare the Communicator to be active
        Self::set_alive(&communicator, true);
        
        // start the main thread
        let com = communicator.clone();
        if let Ok(mut communicator) = communicator.lock() {
            communicator.main_thread = Some(thread::spawn(move || {
                Self::main(&com, bootup_status_send, true);      
            }));
        } else {
            log("erro", "Communicator", "The Communicator got corrupted.");
            return Err(CommunicatorError::FatalError());
        }

        // wait for the bootup status of the main thread
        // true  -> bootup was successful
        // false -> bootup failed
        if let Ok(bootup_status) = bootup_status_receive.recv() {
            if bootup_status == false {
                // the error messages gets printed by the main thread
                return Err(CommunicatorError::FailedBind());
            }
        } else {
            log("erro", "Communicator", "The main thread crashed. The Communicator could not be started.");
            return Err(CommunicatorError::FatalError());
        }
        
        // start the InterCom
        if let Ok(mut ic) = Self::get_intercom(&communicator).lock() {
            ic.start(&communicator.clone());
        } else {
            log("erro", "Communicator", "The InterCom got corrupted. The Communicator could not be started.");
            return Err(CommunicatorError::FatalError());
        }

        Ok(communicator)
    }
    /// Stop the [`Communicator`] and its [`InterCom`].
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                                 | Description               |
    /// |-------------------------------------------|---------------------------|
    /// | `communicator: &Arc<Mutex<Communicator>>` | The Communicator to stop. |
    pub fn stop(communicator: &Arc<Mutex<Communicator>>) {
        let stop_time = Instant::now();
        log("info", "Communicator", "Shutting down...");


        // stop the InterCom
        if let Ok(mut ic) = Self::get_intercom(&communicator).lock() {
            ic.stop();
        }
    
        // give the shutdown command
        Self::set_alive(&communicator, false);

        // wait for all threads to finish
        let main_thread;
        if let Ok(mut communicator) = communicator.lock() {
            main_thread = communicator.main_thread.take().expect("Called stop on non-running thread");
        } else {
            log("erro", "Communicator", "The Communicator got corrupted.");
            unimplemented!("handle PoisonError in case of the Communicator being poisoned")
        }
        main_thread.join().expect("Could not join spawned thread");

        log("info", "Communicator", &format!("Stopped in {} secs!", stop_time.elapsed().as_secs_f64())); 
    }
    /// This method gets used by threads wanting to stop the [`Communicator`] and its [`InterCom`] because of a fatal error.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                                 | Description               |
    /// |-------------------------------------------|---------------------------|
    /// | `communicator: &Arc<Mutex<Communicator>>` | The Communicator to stop. |
    pub fn self_stop(communicator: &Arc<Mutex<Communicator>>) {
        let com = communicator.clone();
        thread::spawn(move || 
            Self::stop(&com)
        );
    }
    /// Restart the [`Communicator`] and its [`InterCom`].
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                                 | Description                                    |
    /// |-------------------------------------------|------------------------------------------------|
    /// | `communicator: &Arc<Mutex<Communicator>>` | The Communicator to stop.                      |
    /// | `mut failcounter: Option<i32>`            | The number of times the restart was attempted. |
    /// | `mut restart_time: Option<Instant>`       | The timestamp of when the restart was started. |
    pub fn restart(communicator: &Arc<Mutex<Communicator>>, mut failcounter: Option<i32>, mut restart_time: Option<Instant>) -> Result<(), CommunicatorError> {
        if let Some(failcounter) = failcounter {
            if failcounter == *Self::get_config(&communicator).max_tries() {
                log("erro", "Communicator", "The maximum number of restart attempts has been reached. The Communicator will no longer attempt to restart.");
                return Err(CommunicatorError::RestartError());
            }
        }
        if let None = restart_time {
            restart_time = Some(Instant::now());
        }
        if let Some(fc) = failcounter {
            failcounter = Some(fc+1);
        } else {
            failcounter = Some(1);
        }
        log("info", "Communicator", "Restarting...");
            
    
        // ### STOPPING ###
    
        
        // stop the InterCom
        if let Ok(mut ic) = Self::get_intercom(&communicator).lock() {
            ic.stop();
        }

        // give the shutdown command
        Self::set_alive(&communicator, false);

        // wait for all threads to finish
        let main_thread;
        if let Ok(mut communicator) = communicator.lock() {
            main_thread = communicator.main_thread.take().expect("Called stop on non-running thread");
        } else {
            log("erro", "Communicator", "The Communicator got corrupted.");
            unimplemented!("handle PoisonError in case of the Communicator being poisoned")
        }
        main_thread.join().expect("Could not join spawned thread");


        // ### STARTING ###
            
        
        // declare the Communicator to be active
        Self::set_alive(&communicator, true);

        // start the main thread
        let communicator_clone = communicator.clone();
        let (bootup_status_send, bootup_status_receive) = mpsc::channel::<bool>();  
        if let Ok(mut communicator) = communicator.lock() {
            (*communicator).main_thread = Some(thread::spawn(move || {
                Self::main(&communicator_clone, bootup_status_send, false);      
            }));
        } else {
            log("erro", "Communicator", "The Communicator got corrupted.");
            unimplemented!("handle PoisonError in case of the Communicator being poisoned")
        }
            
        // wait for the bootup status of the main thread
        // true  -> bootup was successful
        // false -> bootup failed
        if let Ok(bootup_status) = bootup_status_receive.recv() {
            if bootup_status == false {
                // the error messages gets printed by the main thread
                return Err(CommunicatorError::FailedBind());
            }
        } else {
            log("erro", "Communicator", "The main thread crashed. The Communicator will be restarted.");
            log("erro", "Communicator", &format!("This was attempt number {} out of {}", failcounter.unwrap(),Self::get_config(&communicator).max_tries()));
            
            drop(communicator);
            return Self::restart(&communicator.clone(), failcounter, restart_time);
        }
            
        // start the InterCom
        match Self::get_intercom(&communicator).lock() {
            Ok(mut ic) => {
                ic.start(&communicator.clone());
                
                log("info", "Communicator", &format!("Restarted in {} secs!", restart_time.unwrap().elapsed().as_secs_f64()));
                return Ok(());
            }
            Err(_) => {
                log("erro", "Communicator", "The InterCom got corrupted. The Communicator will be restarted.");
                log("erro", "Communicator", &format!("This was attempt number {} out of {}", failcounter.unwrap(), Self::get_config(&communicator).max_tries()));
            }
        }
        return Self::restart(&communicator.clone(), failcounter, restart_time);
    }
    /// This method gets used by threads wanting to restart the [`Communicator`] and its [`InterCom`] because of a fatal error.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                                 | Description               |
    /// |-------------------------------------------|---------------------------|
    /// | `communicator: &Arc<Mutex<Communicator>>` | The Communicator to stop. |
    pub fn self_restart(communicator: &Arc<Mutex<Communicator>>) {
        let com = communicator.clone();
        thread::spawn(move || 
            Self::restart(&com, None, None)
        );
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
    fn main(communicator: &Arc<Mutex<Communicator>>, bootup_status: Sender<bool>, first_start: bool) {
        let getcom = || communicator.clone();

        let mut handlers: Vec<thread::JoinHandle<()>> = vec![];

        let start_time = Instant::now();
        if first_start {
            log("info", "Communicator", "Starting...");
        }

        let mut tries = 0;
        while Self::get_alive(&communicator) {
            tries += 1;

            match TcpListener::bind(Self::get_config(&communicator).addr()) {
                Ok(tcplistener) => {
                    if let Err(err) = tcplistener.set_nonblocking(true) {
                        log("erro", "Communicator", &format!("Failed to activate `non-blocking mode` for the socket server! The Communicator will be restarted. Error: {err}"));
                        log("erro", "Communicator", "The Communicator will be restarted.");
                        
                        Self::self_restart(communicator);
                        return;
                    }

                    if first_start {
                        log("info", "Communicator", &format!("Started in {} secs!", start_time.elapsed().as_secs_f64()));
                    }

                    // the TCPListener got started -> reset the try-counter
                    tries = 0;
                    // the TCPListener got started -> inform the start method of the successful bootup
                    if let Err(_) = bootup_status.send(true) {
                        log("erro", "Communicator", "The thread starting the Communicator got stopped!");
                        log("erro", "Communicator", "The Communicator will now shut down.");

                        
                        if let Ok(communicator) = communicator.lock() {
                            communicator.alive.store(false, Ordering::Relaxed);
                        } else {
                            log("erro", "Communicator", "The Communicator got corrupted.");
                            unimplemented!("handle Communicator corrupted");
                        }
                        break;
                    }

                    // the main loop of the tcplistener-
                    while Self::get_alive(&communicator) {
                        match tcplistener.accept() {
                            Ok(client) => {
                                // create a new thread for the client
                                let com = getcom();
                                handlers.push(thread::spawn(move || {
                                    Self::handler(client.0, client.1, &com);
                                }));
                            }
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => { /* There was no client to be accepted -> ignore this */ }
                            Err(err) => {
                                log("warn", "Communicator", &format!("Found an error while accepting new clients. Error: {err}"));
                                /* It is now the clients responsibility to retry the connection */
                            }
                        }
                        thread::sleep(*Self::get_config(&communicator).refresh_rate());
                    }
                }
                Err(err) => {
                    if tries == *Self::get_config(&communicator).max_tries() {
                        // the TCPListener failed to start -> inform the start method of the failed bootup
                        if let Err(_) = bootup_status.send(false) {
                            log("erro", "Communicator", "The Communicator failed to start.");
                            log("erro", "Communicator", "The thread starting the Communicator got stopped.");

                            return;
                        }

                        log("erro", "Communicator", "The maximum number of tries has been reached.");
                        return;
                    }
                    else {
                        log("warn", "Communicator", &format!("Received an error when trying to bind the socket server. Error: {err}"));
                        log("warn", "Communicator", &format!("This was try number {tries}. 3 seconds till the next one."));
                        thread::sleep(Duration::new(3, 0));
                    } 
                }
            }
        }

        // The Communicator got stopped -> Wait for all handlers to finish before stopping too
        for handler in handlers {
            handler.join().expect("Could not join on stopping handler thread!")
        }
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
    fn handler(mut client: TcpStream, ip: SocketAddr, communicator: &Arc<Mutex<Communicator>>) {
        let id: String;
        let intercom_sender: Sender<Message>;
        let intercom_receiver: Receiver<Message>;
        let mut buffer = vec![0; *Self::get_config(&communicator).buffsize() as usize];
        
        log("info", "Communicator", &format!("A new client has connected using the IP address `{}`.", ip));

        // Register the client at the InterCom
        match Self::register_client(&mut client, ip, Self::get_intercom(&communicator), &Self::get_config(&communicator)) {
            Ok(result) => {
                (id, intercom_sender, intercom_receiver) = result;
            }

            // error messages get send by the method called
            Err(err) if err == CommunicatorError::ConnectionError() => {
                Self::close_connection_ip(&mut client, &ip);
                return;
            }
            // This WILL only handle the FatalError variant
            Err(_) => { 
                Self::close_connection_ip(&mut client, &ip);
                Self::self_restart(communicator);

                return;
            }
        }
        
        // activate the nonblocking mode
        if let Err(err) = client.set_nonblocking(true) {
            log("erro", "Communicator", &format!("Failed to activate the `nonblocking` mode for the client {id}. This Connection will be closed. Error: {err}"));
            Self::close_connection_id(&mut client, &id);
            return;
        }

        // The main loop of the handler
        while Self::get_alive(&communicator) {
            // pass on messages from the InterCom to the client
            match intercom_receiver.try_recv() {
                Ok(msg) => {
                    match client.write(
                        match &msg.to_bytes() {
                            Some(bytes_str) => { bytes_str }
                            None => {
                                log("erro", "Communicator", &format!("Failed to convert the received bytes-string from {id} to a Message. This connection will be closed."));
                                Self::close_connection_id(&mut client, &id);
                                return;
                            }
                        }
                    ) {
                        Ok(n) => {
                            if n == 0 {
                                log("info", "Communicator", &format!("The client {id} disconnected."));
                                Self::close_connection_id(&mut client, &id);
                                return;
                            }
                        }
                        Err(err) => {
                            log("erro", "Communicator", &format!("An error occurred while writing to a message to the client {id}. This connection will be closed. Error: {err}"));
                            Self::close_connection_id(&mut client, &id);
                            return;
                        }
                    }
                }
                Err(err) if err == TryRecvError::Empty => { /* There was no message from the InterCom -> ignore this */ }
                Err(_) => {
                    log("erro", "Communicator", "The connection to the InterCom got interrupted. The Communicator will be restarted.");
                    Self::close_connection_id(&mut client, &id);
                    Self::self_restart(communicator);

                    return;
                }
            }

            // pass on messages from the client
            match client.read(&mut buffer) {
                Ok(n) => {
                    if n == 0 {
                        log("info", "Communicator", &format!("The client {id} disconnected."));
                        Self::close_connection_id(&mut client, &id);
                        return;
                    }

                    let msg: Message;
                    // create a message from the received bytes-string
                    if let Some(result) = Message::from_bytes(buffer.to_vec()) {
                        msg = result;
                    } else {
                        log("erro", "Communicator", &format!("Failed to convert the received bytes-string from {id} to a Message. This connection will be closed."));
                        Self::close_connection_id(&mut client, &id);
                        return;
                    }
                    // send this message to the InterCom
                    if let Err(err) = intercom_sender.send(msg) {
                        log("erro", "Communicator", &format!("An error occurred while writing a message from the client {id} to the InterCom. This connection will be closed. Error: {err}"));
                        Self::close_connection_id(&mut client, &id);
                        return;
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => { /* The client did not sent anything -> Do nothing */ }
                Err(err) => {
                    log("erro", "Communicator", &format!("An error occurred while reading a message from the client {id}. This connection will be closed. Error: {err}"));
                    Self::close_connection_id(&mut client, &id);
                    return;
                }
            }

            thread::sleep(*Self::get_config(&communicator).refresh_rate());
        }
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
    fn register_client(client: &mut TcpStream, ip: SocketAddr, intercom: Arc<Mutex<InterCom>>, config: &Arc<Config>) -> Result<(String, Sender<Message>, Receiver<Message>), CommunicatorError> {
        let client_type: char;
        let id: String;
        let intercom_sender: Sender<Message>;
        let intercom_receiver: Receiver<Message>;
        
        // deactivate the nonblocking mode
        if let Err(err) = client.set_nonblocking(false) {
            log("erro", "Communicator", &format!("Failed to deactivate the `nonblocking` mode for the client {ip}. This Connection will be closed. Error: {err}"));
            return Err(CommunicatorError::ConnectionError());
        }
        
        // get the client type (runner or client)
        match Self::register_client_get_type(client, &ip, config) {
            Ok(ct) => { client_type = ct; }
            Err(err) => { return Err(err); }
        }
        
        // register at the InterCom as a handler
        if let Ok(intercom) = intercom.lock() {
            match (*intercom).add_handler(client_type) {
                Ok(result) => { (id, intercom_sender, intercom_receiver) = result; }
                Err(err) => {
                    log("erro", "Communicator", &format!("Failed to register the client {ip} as handler at the InterCom! This Connection will be closed. Error: {err}"));
                    return Err(CommunicatorError::ConnectionError());
                }
            }
        } else {
            log("erro", "Communicator", "The InterCom got corrupted. The Communicator will be restarted.");
            return Err(CommunicatorError::FatalError());
        };
        log("info", "Communicator", &format!("The client {ip} has been registered as {id}."));

        // inform the client about the end of this registration process
        match client.write(&vec![0]) {
            Ok(n) => {
                if n == 0 {
                    log("info", "Communicator", &format!("The client {id} disconnected."));
                    return Err(CommunicatorError::ConnectionError());
                }
            }
            Err(err) => {
                log("erro", "Communicator", &format!("An error occurred while writing to a message to the client {id}. This connection will be closed. Error: {err}"));
                return Err(CommunicatorError::ConnectionError());
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
    fn register_client_get_type(client: &mut TcpStream, ip: &SocketAddr, config: &Arc<Config>) -> Result<char, CommunicatorError>{
        let mut buffer = vec![0; *config.buffsize() as usize];
        let client_type: char;

        
        match client.write(            
            match &Message::new("get_client_type", "communicator", "", vec![]).to_bytes() {
                Some(bytes_str) => { bytes_str }
                None => {
                    log("erro", "Communicator", &format!("Failed to convert the received bytes-string from {ip} to a Message. This connection will be closed."));
                    return Err(CommunicatorError::ConnectionError());
                }
            }
        ) {
            Ok(n) => {
                if n == 0 {
                    log("info", "Communicator", &format!("The client {ip} disconnected."));
                    return Err(CommunicatorError::ConnectionError());
                }
            }
            Err(err) => {
                log("erro", "Communicator", &format!("An error occurred while writing to a message to the client {ip}. This connection will be closed. Error: {err}"));
                return Err(CommunicatorError::ConnectionError());
            }
        }
        match client.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    log("info", "Communicator", &format!("The client {ip} disconnected."));
                    return Err(CommunicatorError::ConnectionError());
                }
                
                let msg: Message;
                if let Some(m) = Message::from_bytes(buffer.to_vec()) {
                    msg = m;
                } else {
                    log("erro", "Communicator", &format!("Failed to convert the received bytes-string from {ip} to a Message. This connection will be closed."));
                    return Err(CommunicatorError::ConnectionError());
                }
                
                if msg.command() == "get_client_type_response" {
                    if let Some(char) = msg.args()[0].clone().chars().next() {
                        client_type = match char {
                            'r' => char,
                            'c' => char,
                            _ => {
                                log("erro", "Communicator", &format!("Received an invalid client_type from the client {ip}. This connection will be closed."));
                                return Err(CommunicatorError::ConnectionError());
                            }
                        }
                    } else {
                        log("erro", "Communicator", &format!("Received an empty client_type from the client {ip}. This connection will be closed."));
                        return Err(CommunicatorError::ConnectionError());
                    }
                }
                else {
                    log("erro", "Communicator", &format!("Received an invalid first message from the client {ip}. This connection will be closed."));
                    return Err(CommunicatorError::ConnectionError());
                }

            }
            Err(err) => {
                log("erro", "Communicator", &format!("An error occurred while reading a message from the client {ip}. This connection will be closed. Error: {err}"));
                return Err(CommunicatorError::ConnectionError());
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
    fn close_connection_ip(client: &mut TcpStream, ip: &SocketAddr) {
        if let Err(err) = client.shutdown(Shutdown::Both) {
            log("erro", "Communicator", &format!("An error occurred when trying to close the connection to the client {ip}. Error: {err}"));
        }
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
    fn close_connection_id(client: &mut TcpStream, id: &String) {
        if let Err(err) = client.shutdown(Shutdown::Both) {
            log("erro", "Communicator", &format!("An error occurred when trying to close the connection to the client {id}. Error: {err}"));
        }
    }
}