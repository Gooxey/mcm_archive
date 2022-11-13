//! This module contains the [`InterCom struct`](InterCom), which manages the communication between the [`Console`](crate::console::Console) and the [`Communicators handlers`](super::handler).

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Sender, Receiver, RecvTimeoutError};
use std::thread;
use std::collections::HashMap;
use mcm_misc::message::Message;

use crate::error::ChannelError;
use crate::config::Config;

// TODO: Reset the Communication network in case of an FatalError error

/// This struct manages the communication between the [`console`](crate::console::Console) and the [`communicator's`](super::Communicator) [`handlers`](super::handler). \
/// [`Messages`](mcm_misc::message::Message) received from the [`console`](crate::console::Console) will get passed on to the right [`handler`](super::handler),
/// who will send them to the right receiver, and messages received by a [`handler`](super::handler) will get passed on to the [`console`](crate::console::Console),
/// which will execute the command within them.
/// 
/// ## Methods
/// 
/// | Method                                                           | Description                                                           |
/// |------------------------------------------------------------------|-----------------------------------------------------------------------|
/// | [`new(...) -> Self`](InterCom::new)                              | Create a new [`InterCom`] instance.                                   |
/// | [`start(...)`](InterCom::start)                                  | Start the [`InterCom`].                                               |
/// | [`stop(...)`](InterCom::stop)                                    | Stop the [`InterCom`].                                                |
/// | [`add_handler(...) -> Result<...>`](InterCom::add_handler)       | Add a new [`handler`](super::handler) to the [`InterCom`].            |
/// | [`remove_handler(...) -> Result<...>`](InterCom::remove_handler) | Remove an existing [`handler`](super::handler) from the [`InterCom`]. |
/// 
/// ## Requirements
/// 
/// - `mcm_misc` library
pub struct InterCom {
    /// This application's config.
    config: Arc<Config>,
    /// The channel for sending [`messages`](mcm_misc::message::Message) to the [`console`](crate::console::Console).
    sender: Arc<Mutex<Sender<Message>>>,
    /// The channel for receiving [`messages`](mcm_misc::message::Message) from the [`console`](crate::console::Console).
    receiver: Arc<Mutex<Receiver<Message>>>,
    /// A list of every [`handler`](super::handler) id.
    handler_list: Arc<Mutex<Vec<String>>>,
    /// A list of sending and receiving channels for sending and receiving [`messages`](mcm_misc::message::Message) to and from [`handlers`](super::handler). \          
    /// 
    /// | Key                                  | Data -> first element                                       | Data -> second element                                           |
    /// |--------------------------------------|-------------------------------------------------------------|------------------------------------------------------------------|
    /// | the [`handlers'`](super::handler) id | channel to send messages to the [`handler`](super::handler) | channel to receive messages from the [`handler`](super::handler) |
    handlers: Arc<Mutex<HashMap<String, (Sender<Message>, Receiver<Message>)>>>,
    /// The main thread
    main_thread: Option<thread::JoinHandle<()>>,
    /// Controls whether or not the [`main thread`](InterCom::main) is active.
    alive: Arc<AtomicBool>
}
impl InterCom {
    /// Create a new [`InterCom`] instance.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                     | Description                                                                                                                  |
    /// |-------------------------------|------------------------------------------------------------------------------------------------------------------------------|
    /// | `sender: Sender<Message>`     | This channel will be used to pass on [`messages`](mcm_misc::message::Message) to the [`console`](crate::console::Console).   |
    /// | `receiver: Receiver<Message>` | This channel will be used to pass on [`messages`](mcm_misc::message::Message) from the [`console`](crate::console::Console). |
    /// 
    /// ## Usage
    /// 
    /// ```
    /// use mcm_misc::message::Message;
    /// use crate::communicator::intercom::InterCom;
    /// use std::sync::mpsc;
    /// 
    /// # fn main() {
    /// let (ic_tx, _) = mpsc::channel<Message>();
    /// let (_, ic_rx) = mpsc::channel<Message>();
    /// 
    /// let mut myInterCom = InterCom::new(ic_tx, ic_rx);
    /// # }
    /// ```
    pub fn new(sender: Sender<Message>, receiver: Receiver<Message>) -> Self {
        Self {
            config: Arc::new(Config::new()),
            sender: Arc::new(Mutex::new(sender)),
            receiver: Arc::new(Mutex::new(receiver)),
            handler_list: Arc::new(Mutex::new(vec![])),
            handlers: Arc::new(Mutex::new(HashMap::new())),
            main_thread: None,
            alive: Arc::new(AtomicBool::new(false))
        }
    }

    /// Start the [`InterCom`]. \
    /// This will start the [`main thread`](InterCom::main) of the [`InterCom`] and enable it to pass on all incoming [`messages`](mcm_misc::message::Message) to the right receiver.
    /// 
    /// ## Usage
    /// 
    /// ```
    /// # use mcm_misc::message::Message;
    /// # use crate::communicator::intercom::InterCom;
    /// # use std::sync::mpsc;
    /// # fn main() {
    /// // create the InterCom
    /// let (ic_tx, _) = mpsc::channel<Message>();
    /// let (_, ic_rx) = mpsc::channel<Message>();
    /// let mut myInterCom = InterCom::new(ic_tx, ic_rx);
    /// 
    /// // start the InterCom
    /// myInterCom.start();
    /// # }
    /// ```
    pub fn start(&mut self) {
        self.alive.store(true, Ordering::Relaxed);

        let config = self.config.clone();
        let sender = self.sender.clone();
        let receiver = self.receiver.clone();
        let handler_list = self.handler_list.clone();
        let handlers = self.handlers.clone();
        let alive = self.alive.clone();

        self.main_thread = Some(thread::spawn(|| {
            Self::main(
                config,
                sender,
                receiver,
                handler_list,
                handlers,
                alive
            );      
        })); 
    }
    /// Stop the [`InterCom`]. \
    /// This will wait and block the thread until the [`main thread`](InterCom::main) of the [`InterCom`] gets stopped. \
    /// \
    /// Maximum blocking time: ( 1 + amount of [`handlers`](super::handler) ) * [`refresh_rate`](crate::config::Config::refresh_rate)
    /// 
    /// ## Usage
    /// 
    /// ```
    /// # use mcm_misc::message::Message;
    /// # use crate::communicator::intercom::InterCom;
    /// # use std::sync::mpsc;
    /// # fn main() {
    /// // Create and start the InterCom
    /// let (ic_tx, _) = mpsc::channel<Message>();
    /// let (_, ic_rx) = mpsc::channel<Message>();
    /// let mut myInterCom = InterCom::new(ic_tx, ic_rx);
    /// myInterCom.start();
    /// 
    /// // stop the InterCom
    /// myInterCom.stop()
    /// # }
    /// ```
    pub fn stop(&mut self) {
        self.alive.store(false, Ordering::Relaxed);

        self.main_thread
            .take().expect("Called stop on non-running thread")
            .join().expect("Could not join spawned thread");
    }

    /// Add a new [`handler`](super::handler) to the [`InterCom`]. \
    /// This will create new channels for the [`handler`](super::handler) to receive and send [`messages`](mcm_misc::message::Message) to the
    /// [`console`](crate::console::Console).
    /// 
    /// ## Parameters
    /// 
    /// | Parameter            | Description                                           |
    /// |----------------------|-------------------------------------------------------|
    /// | `handler_type: char` | The type of handler requesting a new ID and channels. |
    /// 
    /// ### Handler types
    /// 
    /// | Type | Description                                                                                                      |
    /// |------|------------------------------------------------------------------------------------------------------------------|
    /// | `r`  | This type is used when a [`Runner application`](https://github.com/Gooxey/mcm_runner.git) requests a connection. |
    /// | `c`  | This type is used when a [`Client application`](https://github.com/Gooxey/mcm_client.git) requests a connection. |
    /// 
    /// ## Returns
    /// 
    /// | Return                                             | Description                                                                                        |
    /// |----------------------------------------------------|----------------------------------------------------------------------------------------------------|
    /// | `Ok((String, Sender<Message>, Receiver<Message>))` | The new ID of the [`handler`](super::handler) and its two communication channels will be returned. |
    /// | `Err(ChannelError)`                                | The handler was not able to be added.                                                              |
    /// 
    /// ## Usage
    /// 
    /// ``` 
    /// # use mcm_misc::message::Message;
    /// # use crate::communicator::intercom::InterCom;
    /// # use std::sync::mpsc;
    /// # fn main() {
    /// // Create the InterCom
    /// let (ic_tx, _) = mpsc::channel<Message>();
    /// let (_, ic_rx) = mpsc::channel<Message>();
    /// let mut myInterCom = InterCom::new(ic_tx, ic_rx);
    /// 
    /// // add one handler
    /// let (handler_id, handler_tx, handler_rx) = myInterCom.add_handler("c").unwrap();
    /// # }
    /// ```
    pub fn add_handler(&self, handler_type: char) -> Result<(String, Sender<Message>, Receiver<Message>), ChannelError> {
        // check for invalid types
        match handler_type {
            'r' => {}
            'c' => {}
            _ => {
                return Err(
                    ChannelError::InvalidType(handler_type)
                )
            }
        }
        
        let (tx, handler_recv_channel) = mpsc::channel();
        let (handler_send_channel, rx) = mpsc::channel();
        let id: String;
        
        // add handler to handler_list
        match self.handler_list.lock() {
            Ok(mut handler_list) => {
                let mut i = 0;
                loop {
                    if (*handler_list).contains(&format!("{}{}",handler_type, i)) {
                        i+=1;
                    }
                    else {
                        // valid key found
                        id = format!("{}{}",handler_type, i);
                        // add the id to the list
                        (*handler_list).push(id.clone());
                        break;
                    }
                }
            }
            Err(_) => {
                /* TODO: restart Communication network */
                return Err(
                    ChannelError::FatalError
                )
            }
        }
        // add the channels to the channel storage
        match self.handlers.lock() {
            Ok(mut handlers) => {
                match (*handlers).insert(id.clone(), (handler_send_channel, handler_recv_channel)) {
                    Some(_) => {
                        println!("id: {id}");
                        return Err(
                            ChannelError::DesyncedChannelStorage(id)
                        )
                    }
                    None => { /* Handler got added successfully */ }
                }
            }
            Err(_) => {
                /* TODO: restart Communication network */
                return Err(
                    ChannelError::FatalError
                )
            }
        }    
        Ok((id, tx, rx))
    }
    /// Remove an existing [`handler`](super::handler) from the [`InterCom`]. \
    /// This will remove the existing channels for a specified [`handler`](super::handler) and with that, its ability to receive and send
    /// [`messages`](mcm_misc::message::Message) to the [`console`](crate::console::Console).
    /// 
    /// ## Parameters
    /// 
    /// | Parameter    | Description                                                                          |
    /// |--------------|--------------------------------------------------------------------------------------|
    /// | `id: String` | The ID assigned to the [`handler`](super::handler) when it joined the [`InterCom`]. |
    /// 
    /// ## Returns
    /// 
    /// | Return              | Description                             |
    /// |---------------------|-----------------------------------------|
    /// | `Ok(())`            | The handler was successfully removed.   |
    /// | `Err(ChannelError)` | The handler was not able to be removed. |
    /// 
    /// ## Usage
    /// 
    /// ```
    /// # use mcm_misc::message::Message;
    /// # use crate::communicator::intercom::InterCom;
    /// # use std::sync::mpsc;
    /// # fn main() {
    /// // Create the InterCom
    /// let (ic_tx, _) = mpsc::channel<Message>();
    /// let (_, ic_rx) = mpsc::channel<Message>();
    /// let mut myInterCom = InterCom::new(ic_tx, ic_rx);
    /// 
    /// // add one handler
    /// let (handler_id, handler_tx, handler_rx) = myInterCom.add_handler("c").unwrap();
    /// 
    /// // remove the handler
    /// myInterCom.remove_handler(handler_id).unwrap();
    /// # }
    /// ```
    pub fn remove_handler(&mut self, id: String) -> Result<(), ChannelError> {
        // check for invalid types
        match id.chars().next() {
            Some('r') => {}
            Some('c') => {}
            _ => {
                return Err(
                    ChannelError::InvalidType(id.chars().next().unwrap_or(' '))
                )
            }
        }
        
        // remove handler from the handler_list
        match self.handler_list.lock() {
            Ok(mut handler_list) => {
                let mut i = 0;
                for handler in &*handler_list {
                    if i == (*handler_list).len() {
                        return Err(
                            ChannelError::IDNotFound(id)
                        )
                    }
                    else if handler == &id {
                        (*handler_list).remove(i);
                        break;
                    }
                    i+=1;
                }
            }
            Err(_) => {
                /* TODO: restart Communication network */
                return Err(
                    ChannelError::FatalError
                )
            }
        }
        // remove channels from the channel storage
        match self.handlers.lock() {
            Ok(mut handlers) => {
                match (*handlers).remove(&id) {
                    Some(_) => { /* handler got removed */ }
                    None => {
                        return Err(
                            ChannelError::IDNotFound(id)
                        )
                    }
                }
            }
            Err(_) => {
                /* TODO: restart Communication network */
                return Err(
                    ChannelError::FatalError
                )
            }
        }

        Ok(())
    }

    /// The main thread of the [`InterCom`] which gets invoked by the [`start method`](InterCom::start). \
    /// It will continuously check the receiving channels from the [`console`](crate::console::Console)
    /// and [`handlers`](super::handler) and redirect the received [`messages`](mcm_misc::message::Message) to the [`console`](crate::console::Console)
    /// or the right [`handler`](super::handler). They will then process the contained command or pass on the [`message`](mcm_misc::message::Message) to the right receiver.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                                                                     | Description                                                                                                                                           |
    /// |-------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------|
    /// | `config: Arc<Config>`                                                         | This application's config.                                                                                                                            |
    /// | `sender: Arc<Mutex<Sender<Message>>>`                                         | The channel for sending [`messages`](mcm_misc::message::Message) to the [`console`](crate::console::Console).                                         |
    /// | `receiver: Arc<Mutex<Receiver<Message>>>`                                     | The channel for receiving [`messages`](mcm_misc::message::Message) from the [`console`](crate::console::Console).                                     |
    /// | `handler_list: Arc<Mutex<Vec<String>>>`                                       | A list of every [`handler`](super::handler) id.                                                                                                       |
    /// | `handlers: Arc<Mutex<HashMap<String, (Sender<Message>, Receiver<Message>)>>>` | A list of sending and receiving channels for sending and receiving [`messages`](mcm_misc::message::Message) to and from [`handlers`](super::handler). |
    /// | `alive: Arc<AtomicBool>`                                                      | Controls whether or not the [`main thread`](InterCom::main) is active.                                                                                |
    fn main(
        config: Arc<Config>,
        sender: Arc<Mutex<Sender<Message>>>,
        receiver: Arc<Mutex<Receiver<Message>>>,
        handler_list: Arc<Mutex<Vec<String>>>,
        handlers: Arc<Mutex<HashMap<String, (Sender<Message>, Receiver<Message>)>>>,
        alive: Arc<AtomicBool>
    ) {
        while alive.load(Ordering::Relaxed) {
            // check if the Console wants to send something and then pass it on to the right receiver if possible
            match receiver.lock() {
                Ok(rx) => {
                    match (*rx).recv_timeout(config.refresh_rate().to_owned()) {
                        Ok(msg) => {
                            let receiver = msg.receiver();

                            // get the channel to send to the receiver handler
                            match handlers.lock() {
                                Ok(hs) => {
                                    match (*hs).get(receiver) {
                                        Some(handler) => {
                                            match handler.0.send(msg) {
                                                Ok(_) => { /* message got sent */ }
                                                Err(_) => { /* handler gone and deleted => ignore this message */ }
                                            }
                                        }
                                        None => { /* handler gone and deleted => ignore this message */}
                                    }
                                }
                                Err(_) => { /* Try next time */ }
                            }
                        }
                        Err(e) => {
                            match e {
                                RecvTimeoutError::Timeout => { /* There is no message currently waiting to be send */}
                                RecvTimeoutError::Disconnected => {
                                    // TODO: restart Communication network
                                }
                            }
                        }
                    }
                }
                Err(_) => { /* This mutex is only used by this function. There cant be a panic because on any critical error this communication network will restart */}
            }
            // check if any handler wants to send something and then pass it on to the Console
            match handlers.lock() {
                Ok(hr) => {
                    let mut handler_list_save: Vec<String> = vec![];

                    // get the handler list
                    match handler_list.lock() {
                        Ok(hl) => {
                            handler_list_save = (*hl).clone();
                        }
                        Err(_) => { /* TODO: restart Communication network */ }
                    }

                    for handler_id in &handler_list_save {
                        match (*hr).get(handler_id) {
                            Some(rx) => {
                                match rx.1.recv_timeout(config.refresh_rate().to_owned()) {
                                    Ok(msg) => {
                                        // send to Console because all incoming messages have to be processed by the Console
                                        match sender.lock() {
                                            Ok(tx) => {
                                                match (*tx).send(msg) {
                                                    Ok(_) => { /* message got sent */}
                                                    Err(_) => { /* handler gone and deleted => ignore this message */ }
                                                }
                                            }
                                            Err(_) => { /* Try next time */ }
                                        }
                                    }
                                    Err(e) => {
                                        match e {
                                            RecvTimeoutError::Timeout => { /* There is no message currently waiting to be send */}
                                            RecvTimeoutError::Disconnected => {
                                                // TODO: restart Communication network
                                            }
                                        }
                                    }
                                }
                            }
                            None => { /* handler gone and deleted => ignore this id */ }
                        }
                    }
                }
                Err(_) => { /* Try next time */ }
            }
        }
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    // InterCom__add_handler tests
    #[test]
    fn InterCom__add_handler__valid_chars() {
        let (ic_tx, _) = mpsc::channel::<Message>();
        let (_, ic_rx) = mpsc::channel::<Message>();
        let myInterCom = InterCom::new(ic_tx, ic_rx);

        match myInterCom.add_handler('r') {
            Ok(r) => {
                let id = r.0;

                assert!(myInterCom.handler_list.lock().unwrap().contains(&format!("{id}")), "The given id {} is missing in the handler_list.", format!("{id}"));
                assert!(myInterCom.handlers.lock().unwrap().contains_key(&format!("{id}")), "The given key {} is missing in handlers.", format!("{id}"));
            }
            Err(e) => {
                assert!(false, "{}", e);
            }
        }
        match myInterCom.add_handler('c') {
            Ok(r) => {
                let id = r.0;

                assert!(myInterCom.handler_list.lock().unwrap().contains(&format!("{id}")), "The given id {} is missing in the handler_list.", format!("{id}"));
                assert!(myInterCom.handlers.lock().unwrap().contains_key(&format!("{id}")), "The given key {} is missing in handlers.", format!("{id}"));
            }
            Err(e) => {
                assert!(false, "{}", e);
            }
        }
    }
    #[test]
    fn InterCom__add_handler__invalid_chars() {
        let (ic_tx, _) = mpsc::channel::<Message>();
        let (_, ic_rx) = mpsc::channel::<Message>();
        let myInterCom = InterCom::new(ic_tx, ic_rx);

        match myInterCom.add_handler('d') {
            Ok(r) => {
                let id = r.0;
                
                assert!(!myInterCom.handler_list.lock().unwrap().contains(&format!("{id}")), "The invalid id {} was found in the handler_list.", format!("{id}"));
                assert!(!myInterCom.handlers.lock().unwrap().contains_key(&format!("{id}")), "The invalid key {} was found in handlers.", format!("{id}"));
            }
            Err(e) => {
                match e {
                    ChannelError::InvalidType(_) => {
                        assert!(true)
                    }
                    _ => {
                        assert!(false, "{}", e)
                    }
                }
            }
        }
        match myInterCom.add_handler(' ') {
            Ok(r) => {
                let id = r.0;

                assert!(!myInterCom.handler_list.lock().unwrap().contains(&format!("{id}")), "The invalid id {} was found in the handler_list.", format!("{id}"));
                assert!(!myInterCom.handlers.lock().unwrap().contains_key(&format!("{id}")), "The invalid key {} was found in handlers.", format!("{id}"));
            }
            Err(e) => {
                match e {
                    ChannelError::InvalidType(_) => {
                        assert!(true)
                    }
                    _ => {
                        assert!(false, "{}", e)
                    }
                }
            }
        }
    }

    // InterCom__remove_handler tests
    #[test]
    fn InterCom__remove_handler__existing_id() {
        let (ic_tx, _) = mpsc::channel::<Message>();
        let (_, ic_rx) = mpsc::channel::<Message>();
        let mut myInterCom = InterCom::new(ic_tx, ic_rx);

        let (id, _, _) = myInterCom.add_handler('r').unwrap();

        match myInterCom.remove_handler(id.clone()) {
            Ok(_) => {
                assert!(!myInterCom.handler_list.lock().unwrap().contains(&format!("{id}")), "The given id {} is still in the handler_list.", format!("{id}"));
                assert!(!myInterCom.handlers.lock().unwrap().contains_key(&format!("{id}")), "The given key {} is still in handlers.", format!("{id}"));
            }
            Err(e) => {
                assert!(false, "{}", e);
            }
        }
    }
    #[test]
    fn InterCom__remove_handler__nonexisting_id() {
        let (ic_tx, _) = mpsc::channel::<Message>();
        let (_, ic_rx) = mpsc::channel::<Message>();
        let mut myInterCom = InterCom::new(ic_tx, ic_rx);

        match myInterCom.remove_handler("r6".to_owned()) {
            Ok(_) => {
                assert!(false, "Expected the error: ChannelError::IDNotFound.");
            }
            Err(e) => {
                match e {
                    ChannelError::IDNotFound(_) => {
                        assert!(true)
                    }
                    _ => {
                        assert!(false, "Expected the error: ChannelError::IDNotFound. Found: {e}");
                    }
                }
            }
        }
    }
    #[test]
    fn InterCom__remove_handler__invalid_id() {
        let (ic_tx, _) = mpsc::channel::<Message>();
        let (_, ic_rx) = mpsc::channel::<Message>();
        let mut myInterCom = InterCom::new(ic_tx, ic_rx);

        match myInterCom.remove_handler("d0".to_owned()) {
            Ok(_) => {
                assert!(false, "Expected the error: ChannelError::InvalidType.");
            }
            Err(e) => {
                match e {
                    ChannelError::InvalidType(_) => {
                        assert!(true)
                    }
                    _ => {
                        assert!(false, "Expected the error: ChannelError::InvalidType. Found: {e}");
                    }
                }
            }
        }
        match myInterCom.remove_handler(" ".to_owned()) {
            Ok(_) => {
                assert!(false, "Expected the error: ChannelError::InvalidType.");
            }
            Err(e) => {
                match e {
                    ChannelError::InvalidType(_) => {
                        assert!(true)
                    }
                    _ => {
                        assert!(false, "Expected the error: ChannelError::InvalidType. Found: {e}");
                    }
                }
            }
        }
    }

    // InterCom start/stop tests
    #[test]
    fn InterCom__start() {
        let (ic_tx, _) = mpsc::channel::<Message>();
        let (_, ic_rx) = mpsc::channel::<Message>();
        let mut myInterCom: InterCom = InterCom::new(ic_tx, ic_rx);

        myInterCom.start();

        // check if thread got created and alive status set to true
        match myInterCom.main_thread {
            Some(_) => {
                assert!(true)
            }
            None => {
                assert!(false, "Expected thread to be created and saved to the `main_thread` field. Found nothing.")
            }
        }
        assert_eq!(myInterCom.alive.load(Ordering::Relaxed), true, "Expected `alive` field to be set to `true`.");
    }
    #[test]
    fn InterCom__stop() {
        let (ic_tx, _) = mpsc::channel::<Message>();
        let (_, ic_rx) = mpsc::channel::<Message>();
        let mut myInterCom: InterCom = InterCom::new(ic_tx, ic_rx);

        myInterCom.start();
        myInterCom.stop();

        // check if thread got deleted and alive status set to false
        match myInterCom.main_thread {
            Some(_) => {
                assert!(false, "Expected thread to be deleted. Found a thread in the `main_thread` field.")
            }
            None => {
                assert!(true)
            }
        }
        assert_eq!(myInterCom.alive.load(Ordering::Relaxed), false, "Expected `alive` field to be set to `false`.");
    }

    // InterCom tests
    #[test]
    fn InterCom__Console_to_handler() {
        use std::time::Duration;

        let (ic_tx, _) = mpsc::channel::<Message>();
        let (tx, ic_rx) = mpsc::channel::<Message>();
        let mut myInterCom: InterCom = InterCom::new(ic_tx, ic_rx);

        myInterCom.start();

        let (id1, _, rx1) = myInterCom.add_handler('c').unwrap();
        let (id2, _, rx2) = myInterCom.add_handler('c').unwrap();

        // send messages
        tx.send(Message::new("test message to client 1", "", &id1, vec![])).unwrap();
        tx.send(Message::new("test message to client 2", "", &id2, vec![])).unwrap();

        // receive messages

        // client 1
        match rx1.recv_timeout(Duration::new(1,0)) {
            Ok(data) => {
                assert_eq!(data.command(), &"test message to client 1".to_owned(), "Client 1 received the wrong message.")
            }
            Err(e) => {
                match e {
                    RecvTimeoutError::Timeout => {
                        assert!(false, "The Message 1 did not get received by client 1")
                    }
                    _ => {
                        assert!(false, "{e}")
                    }
                }
            }
        }

        // client 2
        match rx2.recv_timeout(Duration::new(1,0)) {
            Ok(data) => {
                assert_eq!(data.command(), &"test message to client 2".to_owned(), "Client 2 received the wrong message.")
            }
            Err(e) => {
                match e {
                    RecvTimeoutError::Timeout => {
                        assert!(false, "The Message 2 did not get received by client 2")
                    }
                    _ => {
                        assert!(false, "{e}")
                    }
                }
            }
        }
    }
    #[test]
    fn InterCom__handler_to_Console() {
        use std::time::Duration;

        let (ic_tx, rx) = mpsc::channel::<Message>();
        let (_, ic_rx) = mpsc::channel::<Message>();
        let mut myInterCom: InterCom = InterCom::new(ic_tx, ic_rx);

        myInterCom.start();

        let (id1, tx1, _) = myInterCom.add_handler('c').unwrap();
        let (id2, tx2, _) = myInterCom.add_handler('c').unwrap();

        // send messages
        tx1.send(Message::new("test message from client 1", &id1, "some_thread", vec![])).unwrap();
        tx2.send(Message::new("test message from client 2", "", &id2, vec![])).unwrap();

        // receive messages
        match rx.recv_timeout(Duration::new(1,0)) {
            Ok(data) => {
                assert_eq!(data.command(), &"test message from client 1".to_owned(), "Console received the wrong message.")
            }
            Err(e) => {
                match e {
                    RecvTimeoutError::Timeout => {
                        assert!(false, "The Message 1 did not get received by Console")
                    }
                    _ => {
                        assert!(false, "{e}")
                    }
                }
            }
        }        
        match rx.recv_timeout(Duration::new(1,0)) {
            Ok(data) => {
                assert_eq!(data.command(), &"test message from client 2".to_owned(), "Console received the wrong message.")
            }
            Err(e) => {
                match e {
                    RecvTimeoutError::Timeout => {
                        assert!(false, "The Message 2 did not get received by Console")
                    }
                    _ => {
                        assert!(false, "{e}")
                    }
                }
            }
        }
    }
}