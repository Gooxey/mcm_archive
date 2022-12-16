//! This module contains the [`InterCom struct`](InterCom), which manages the communication between the [`Console`](crate::console::Console) and the [`Communicators handlers`](super::Communicator::handler).

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Sender, Receiver, TryRecvError};
use std::thread;
use std::collections::HashMap;
use mcm_misc::log::log;
use mcm_misc::message::Message;

use crate::error::ChannelError;
use crate::config::Config;

use super::Communicator;


mod tests;


/// This struct manages the communication between the [`console`](crate::console::Console) and the [`communicator's`](super::Communicator) [`handlers`](super::Communicator::handler). \
/// [`Messages`](mcm_misc::message::Message) received from the [`console`](crate::console::Console) will get passed on to the right [`handler`](super::Communicator::handler),
/// who will send them to the right receiver, and messages received by a [`handler`](super::Communicator::handler) will get passed on to the [`console`](crate::console::Console),
/// which will execute the command within them.
/// 
/// ## Methods
/// 
/// | Method                                                           | Description                                                                         |
/// |------------------------------------------------------------------|-------------------------------------------------------------------------------------|
/// | [`new(...) -> Self`](InterCom::new)                              | Create a new [`InterCom`] instance.                                                 |
/// | [`start(...)`](InterCom::start)                                  | Start the [`InterCom`].                                                             |
/// | [`stop(...)`](InterCom::stop)                                    | Stop the [`InterCom`].                                                              |
/// | [`add_handler(...) -> Result<...>`](InterCom::add_handler)       | Add a new [`handler`](super::Communicator::handler) to the [`InterCom`].            |
/// | [`remove_handler(...) -> Result<...>`](InterCom::remove_handler) | Remove an existing [`handler`](super::Communicator::handler) from the [`InterCom`]. |
pub struct InterCom {
    /// This application's config.
    config: Arc<Config>,
    /// The channel for sending [`messages`](mcm_misc::message::Message) to the [`console`](crate::console::Console).
    sender: Arc<Mutex<Sender<Message>>>,
    /// The channel for receiving [`messages`](mcm_misc::message::Message) from the [`console`](crate::console::Console).
    receiver: Arc<Mutex<Receiver<Message>>>,
    /// A list of every [`handler`](super::Communicator::handler) id.
    handler_list: Arc<Mutex<Vec<String>>>,
    /// A list of sending and receiving channels for sending and receiving [`messages`](mcm_misc::message::Message) to and from [`handlers`](super::Communicator::handler). \          
    /// 
    /// | Key                                                | Data -> first element                                                     | Data -> second element                                                         |
    /// |----------------------------------------------------|---------------------------------------------------------------------------|--------------------------------------------------------------------------------|
    /// | the [`handlers'`](super::Communicator::handler) id | channel to send messages to the [`handler`](super::Communicator::handler) | channel to receive messages from the [`handler`](super::Communicator::handler) |
    handlers: Arc<Mutex<HashMap<String, (Sender<Message>, Receiver<Message>)>>>,
    /// The main thread
    main_thread: Option<thread::JoinHandle<()>>,
    /// Controls whether or not the [`main thread`](InterCom::main) is active.
    alive: Arc<AtomicBool>,
    /// The Communicator using this InterCom.
    communicator: Option<Arc<Mutex<Communicator>>>
}
impl InterCom {
    /// Create a new [`InterCom`] instance.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                     | Description                                                                                                                  |
    /// |-------------------------------|------------------------------------------------------------------------------------------------------------------------------|
    /// | `sender: Sender<Message>`     | This channel will be used to pass on [`messages`](mcm_misc::message::Message) to the [`console`](crate::console::Console).   |
    /// | `receiver: Receiver<Message>` | This channel will be used to receive [`messages`](mcm_misc::message::Message) from the [`console`](crate::console::Console). |
    pub fn new(config: Arc<Config>, sender: Sender<Message>, receiver: Receiver<Message>) -> Self {
        Self {
            config,
            sender: Arc::new(Mutex::new(sender)),
            receiver: Arc::new(Mutex::new(receiver)),
            handler_list: Arc::new(Mutex::new(vec![])),
            handlers: Arc::new(Mutex::new(HashMap::new())),
            main_thread: None,
            alive: Arc::new(AtomicBool::new(false)),
            communicator: None
        }
    }

    /// Start the [`InterCom`]. \
    /// This will start the [`main thread`](InterCom::main) of the [`InterCom`] and enable it to pass on all incoming [`messages`](mcm_misc::message::Message) to the right receiver.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                                | Description                           |
    /// |------------------------------------------|---------------------------------------|
    /// | `communicator: Arc<Mutex<Communicator>>` | The Communicator using this InterCom. |
    pub fn start(&mut self, communicator: &Arc<Mutex<Communicator>>) {
        self.communicator = Some(communicator.clone());
        
        self.alive.store(true, Ordering::Relaxed);

        let config = self.config.clone();
        let sender = self.sender.clone();
        let receiver = self.receiver.clone();
        let handler_list = self.handler_list.clone();
        let handlers = self.handlers.clone();
        let alive = self.alive.clone();
        let communicator = self.communicator.clone().unwrap();

        self.main_thread = Some(thread::spawn(|| {
            Self::main(
                config,
                sender,
                receiver,
                handler_list,
                handlers,
                alive,
                communicator
            );      
        }));
    }
    /// Stop the [`InterCom`]. \
    /// This will wait and block the thread until the [`main thread`](InterCom::main) of the [`InterCom`] gets stopped. \
    /// \
    /// Maximum blocking time: ( 1 + amount of [`handlers`](super::Communicator::handler) ) * [`refresh_rate`](crate::config::Config::refresh_rate)
    pub fn stop(&mut self) {
        self.alive.store(false, Ordering::Relaxed);

        if let Some(main_thread) = self.main_thread.take() {
            main_thread.join().expect("Could not join spawned thread");
        }
    }

    /// Add a new [`handler`](super::Communicator::handler) to the [`InterCom`]. \
    /// This will create new channels for the [`handler`](super::Communicator::handler) to receive and send [`messages`](mcm_misc::message::Message) to the
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
    /// | Return                                             | Description                                                                                                      |
    /// |----------------------------------------------------|------------------------------------------------------------------------------------------------------------------|
    /// | `Ok((String, Sender<Message>, Receiver<Message>))` | The new ID of the [`handler`](super::Communicator::handler) and its two communication channels will be returned. |
    /// | `Err(ChannelError)`                                | The handler was not able to be added.                                                                            |
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
        
        let (intercom_send, handler_receive) = mpsc::channel();
        let (handler_send, intercom_receive) = mpsc::channel();
        let id: String;
        
        // add handler to handler_list
        if let Ok(mut handler_list) = self.handler_list.lock() {
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

            // add the channels to the channel storage
            if let Ok(mut handlers) = self.handlers.lock() {
                if let Some(_) = (*handlers).insert(id.clone(), (handler_send, handler_receive)) {
                    return Err(ChannelError::DesyncedChannelStorage(id))
                }
            } else {
                log("erro", "InterCom", &format!("The handlers map got corrupted. The Communicator will be restarted."));
                Communicator::self_restart(&self.communicator.as_ref().unwrap());

                return Err(ChannelError::FatalError)
            }
        } else {
            log("erro", "InterCom", &format!("The handler_list got corrupted. The Communicator will be restarted."));
            Communicator::self_restart(&self.communicator.as_ref().unwrap());

            return Err(ChannelError::FatalError)
        }
        
        Ok((id, intercom_send, intercom_receive))
    }
    /// Remove an existing [`handler`](super::Communicator::handler) from the [`InterCom`]. \
    /// This will remove the existing channels for a specified [`handler`](super::Communicator::handler) and with that, its ability to receive and send
    /// [`messages`](mcm_misc::message::Message) to the [`console`](crate::console::Console).
    /// 
    /// ## Parameters
    /// 
    /// | Parameter    | Description                                                                                       |
    /// |--------------|---------------------------------------------------------------------------------------------------|
    /// | `id: String` | The ID assigned to the [`handler`](super::Communicator::handler) when it joined the [`InterCom`]. |
    /// 
    /// ## Returns
    /// 
    /// | Return              | Description                             |
    /// |---------------------|-----------------------------------------|
    /// | `Ok(())`            | The handler was successfully removed.   |
    /// | `Err(ChannelError)` | The handler was not able to be removed. |
    pub fn remove_handler(&mut self, id: String) -> Result<(), ChannelError> {
        // check for invalid types
        match id.chars().next() {
            Some('r') => {}
            Some('c') => {}
            _ => {
                return Err(ChannelError::InvalidType(id.chars().next().unwrap_or(' ')))
            }
        }
        
        // remove handler from the handler_list
        if let Ok(mut handler_list) = self.handler_list.lock() {
            let mut i = 0;
            for handler in &*handler_list {
                if i == (*handler_list).len() {
                    return Err(ChannelError::IDNotFound(id));
                }
                else if handler == &id {
                    (*handler_list).remove(i);
                    break;
                }
                i+=1;
            }

            // remove channels from the channel storage
            if let Ok(mut handlers) = self.handlers.lock() {
                if let None = (*handlers).remove(&id) {
                    return Err(ChannelError::IDNotFound(id));
                }
            } else {
                log("erro", "InterCom", &format!("The handlers map got corrupted. The Communicator will be restarted."));
                Communicator::self_restart(&self.communicator.as_ref().unwrap());

                return Err(ChannelError::FatalError);
            }
        } else {
            log("erro", "InterCom", &format!("The handler_list got corrupted. The Communicator will be restarted."));
            Communicator::self_restart(&self.communicator.as_ref().unwrap());

            return Err( ChannelError::FatalError);
        }

        Ok(())
    }
    /// A version of the [`remove_handler`](InterCom::remove_handler) method used by threads that are not able to provide the necessary InterCom Object.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                                                              | Description                                                                                                                                                         |
    /// |------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------|
    /// | `handler_list: &mut Vec<String>`                                       | A list of every [`handler`](super::Communicator::handler) id.                                                                                                       |
    /// | `handlers: &mut HashMap<String, (Sender<Message>, Receiver<Message>)>` | A list of sending and receiving channels for sending and receiving [`messages`](mcm_misc::message::Message) to and from [`handlers`](super::Communicator::handler). |
    /// | `id: String`                                                           | The ID assigned to the [`handler`](super::Communicator::handler) when it joined the [`InterCom`].                                                                   |
    /// 
    /// ## Returns
    /// 
    /// | Return              | Description                             |
    /// |---------------------|-----------------------------------------|
    /// | `Ok(())`            | The handler was successfully removed.   |
    /// | `Err(ChannelError)` | The handler was not able to be removed. |
    fn remove_handler_intern(handler_list: &mut Vec<String>, handlers: &mut HashMap<String, (Sender<Message>, Receiver<Message>)>, id: &String) -> Result<(), ChannelError> {
        // check for invalid types
        match id.chars().next() {
            Some('r') => {}
            Some('c') => {}
            _ => {
                return Err(ChannelError::InvalidType(id.chars().next().unwrap_or(' ')))
            }
        }
        
        // remove handler from the handler_list
        let mut i = 0;
        for handler in handler_list.clone() {
            if i == handler_list.len() {
                return Err(ChannelError::IDNotFound(id.clone()));
            }
            else if handler == *id {
                handler_list.remove(i);
                break;
            }
            i+=1;
        }

        // remove channels from the channel storage
        if let None = handlers.remove(id) {
            return Err(ChannelError::IDNotFound(id.clone()));
        }

        Ok(())
    }
    
    /// The main thread of the [`InterCom`] which gets invoked by the [`start method`](InterCom::start). \
    /// It will continuously check the receiving channels from the [`console`](crate::console::Console)
    /// and [`handlers`](super::Communicator::handler) and redirect the received [`messages`](mcm_misc::message::Message) to the [`console`](crate::console::Console)
    /// or the right [`handler`](super::Communicator::handler). They will then process the contained command or pass on the [`message`](mcm_misc::message::Message) to the right receiver.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                                                                     | Description                                                                                                                                                         |
    /// |-------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------|
    /// | `config: Arc<Config>`                                                         | This application's config.                                                                                                                                          |
    /// | `sender: Arc<Mutex<Sender<Message>>>`                                         | The channel for sending [`messages`](mcm_misc::message::Message) to the [`console`](crate::console::Console).                                                       |
    /// | `receiver: Arc<Mutex<Receiver<Message>>>`                                     | The channel for receiving [`messages`](mcm_misc::message::Message) from the [`console`](crate::console::Console).                                                   |
    /// | `handler_list: Arc<Mutex<Vec<String>>>`                                       | A list of every [`handler`](super::Communicator::handler) id.                                                                                                       |
    /// | `handlers: Arc<Mutex<HashMap<String, (Sender<Message>, Receiver<Message>)>>>` | A list of sending and receiving channels for sending and receiving [`messages`](mcm_misc::message::Message) to and from [`handlers`](super::Communicator::handler). |
    /// | `alive: Arc<AtomicBool>`                                                      | Controls whether or not the [`main thread`](InterCom::main) is active.                                                                                              |
    /// | `communicator: Arc<Mutex<Communicator>>`                                      | The Communicator using this InterCom.                                                                                                                               |
    fn main(
        config: Arc<Config>,
        sender: Arc<Mutex<Sender<Message>>>,
        receiver: Arc<Mutex<Receiver<Message>>>,
        handler_list: Arc<Mutex<Vec<String>>>,
        handlers: Arc<Mutex<HashMap<String, (Sender<Message>, Receiver<Message>)>>>,
        alive: Arc<AtomicBool>,
        communicator: Arc<Mutex<Communicator>>
    ) {
        while alive.load(Ordering::Relaxed) {
            // check if the Console wants to send something and then pass it on to the right receiver if possible
            if let Ok(rx) = receiver.lock() {
                match (*rx).try_recv() {
                    Ok(msg) => {
                        let receiver = msg.receiver();
            
                        // get the channel to send to the receiver handler
                        if let Ok(hs) = handlers.lock() {
                            if let Some(handler) = (*hs).get(receiver) {
                                if let Ok(_) = handler.0.send(msg) { /* message got sent */ }
                            }
                        } else {
                            log("erro", "InterCom", &format!("The list containing all handler channels got corrupted. The Communicator will be restarted."));
                            Communicator::self_restart(&communicator);

                            return;
                        }
                    }
                    Err(err) if err == TryRecvError::Empty => { /* There is no message currently waiting to be send */ }
                    Err(_) => {
                        log("erro", "InterCom", &format!("The Console disconnected! The Communicator will shut down."));
                        Communicator::self_stop(&communicator);

                        return;
                    }
                }
            } else {
                log("erro", "InterCom", &format!("The channel for receiving messages from the Console got corrupted. The Communicator will be restarted."));
                Communicator::self_restart(&communicator);

                return;
            }

            // check if any handler wants to send something and then pass it on to the Console
            if let Ok(mut hr) = handlers.lock() {
                if let Ok(mut hl) = handler_list.lock() {
                    for handler_id in &*hl.clone() {
                        if let Some(rx) = (*hr).get(handler_id) {
                            match rx.1.try_recv() {
                                Ok(msg) => {                                        
                                    // send to Console because all incoming messages have to be processed by the Console
                                    if let Ok(tx) = sender.lock() {
                                        if let Ok(_) = (*tx).send(msg) { /* message got sent */ }
                                    } else {
                                        log("erro", "InterCom", &format!("The channel for sending messages to the Console got corrupted. The Communicator will be restarted."));
                                        Communicator::self_restart(&communicator);
    
                                        return;
                                    }
                                }
                                Err(err) if err == TryRecvError::Empty => { /* There is no message currently waiting to be send */ }
                                Err(_) => {
                                    log("erro", "InterCom", &format!("The handler {handler_id} disconnected."));
                                    if let Err(_) = Self::remove_handler_intern(&mut hl, &mut hr, &handler_id) {
                                        /* Do nothing because it must already have been removed */
                                    }
                                }
                            }
                        } else {
                            log("erro", "InterCom", &format!("The handler list did not match the handler_id list. The Communicator will be restarted."));
                            Communicator::self_restart(&communicator);
    
                            return;
    
                        }
                    }
                } else {
                    log("erro", "InterCom", &format!("The list containing all handler ids got corrupted. The Communicator will be restarted."));
                    Communicator::self_restart(&communicator);

                    return;
                }  
            } else {
                log("erro", "InterCom", &format!("The list containing all handler channels got corrupted. The Communicator will be restarted."));
                Communicator::self_restart(&communicator);

                return;
            }

            thread::sleep(*config.refresh_rate());
        }
    }
}