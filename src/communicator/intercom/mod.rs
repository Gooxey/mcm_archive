//! This module contains the [`InterCom struct`](InterCom), which manages the communication between the [`Console`](crate::console::Console) and the [`Communicators handlers`](super::Communicator::handler).

use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::mpsc::{self, Sender, Receiver, TryRecvError};
use std::thread;
use std::collections::HashMap;
use mcm_misc::log;
use mcm_misc::message::Message;
use mcm_misc::config_trait::ConfigTrait;
use mcm_misc::concurrent_class::ConcurrentClass;
use mcm_misc::mcmanage_error::MCManageError;

use intercom_error::InterComError;

use super::Communicator;


mod tests;
pub mod intercom_error;


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
pub struct InterCom<C: ConfigTrait> {
    /// This application's config.
    config: Arc<C>,
    /// The channel for sending [`messages`](mcm_misc::message::Message) to the [`console`](crate::console::Console).
    sender: Sender<Message>,
    /// The channel for receiving [`messages`](mcm_misc::message::Message) from the [`console`](crate::console::Console).
    receiver: Option<Receiver<Message>>,
    /// A list of every [`handler`](super::Communicator::handler) id.
    handler_list: Vec<String>,
    /// A list of sending and receiving channels for sending and receiving [`messages`](mcm_misc::message::Message) to and from [`handlers`](super::Communicator::handler). \          
    /// 
    /// | Key                                                | Data -> first element                                                     | Data -> second element                                                         |
    /// |----------------------------------------------------|---------------------------------------------------------------------------|--------------------------------------------------------------------------------|
    /// | the [`handlers'`](super::Communicator::handler) id | channel to send messages to the [`handler`](super::Communicator::handler) | channel to receive messages from the [`handler`](super::Communicator::handler) |
    handlers: HashMap<String, (Sender<Message>, Receiver<Message>)>,
    /// The main thread
    main_thread: Option<thread::JoinHandle<()>>,
    /// Controls whether or not the [`main thread`](InterCom::main) is active.
    alive: bool,
    /// The Communicator using this InterCom.
    communicator: Option<Arc<Mutex<Communicator<C>>>>
}
impl<C: ConfigTrait> ConcurrentClass<InterCom<C>, C> for InterCom<C> {
    fn get_config_unlocked(class_lock: &MutexGuard<InterCom<C>>) -> Arc<C> {
        class_lock.config.clone()
    }
    fn get_name_unlocked(_: &MutexGuard<InterCom<C>>) -> String {
        "InterCom".to_string()
    }
    fn get_name_poison_error(_: &MutexGuard<InterCom<C>>) -> String {
        "InterCom".to_string()
    }
    fn get_default_state(class_lock: &mut MutexGuard<InterCom<C>>) -> InterCom<C> {
        InterCom {
            config: class_lock.config.clone(),
            sender: class_lock.sender.clone(),
            receiver: class_lock.receiver.take(),
            handler_list: vec![],
            handlers: HashMap::new(),
            main_thread: None,
            alive: false,
            communicator: class_lock.communicator.clone()
        }
    }
    fn start(class: &Arc<Mutex<InterCom<C>>>, log_messages: bool) -> Result<(), MCManageError> {
        let mut class_lock;
        if let Some(lock) = Self::get_lock_pure(class, false) {
            class_lock = lock;
        } else {
            if log_messages { log!("erro", "InterCom", "This InterCom got corrupted. It will not start."); }
            Self::reset(&class);
            return Err(MCManageError::FatalError);
        }

        if let None = class_lock.communicator {
            if log_messages { log!("erro", "InterCom", "The Communicator has not yet been set."); }
            return Err(MCManageError::NotReady);
        }
        
        class_lock.alive = true;

        let class_clone = class.clone();
        class_lock.main_thread = Some(thread::spawn(move || {
            Self::main(class_clone);      
        }));

        Ok(())
    }
    fn stop(class: &Arc<Mutex<InterCom<C>>>, log_messages: bool) -> Result<(), MCManageError> {
        let mut class_lock;
        if let Some(lock) = Self::get_lock_pure(class, false) {
            class_lock = lock;
        } else {
            if log_messages { log!("erro", "InterCom", "This InterCom got corrupted. It will reset."); }
            Self::reset(&class);
            return Err(MCManageError::FatalError);
        }
        
        class_lock.alive = false;

        if let Some(main_thread) = class_lock.main_thread.take() {
            drop(class_lock);
            main_thread.join().expect("Could not join spawned thread");
        }

        Ok(())
    }
}
impl<C: ConfigTrait> InterCom<C> {
    /// Create a new [`InterCom`] instance.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                     | Description                                                                                                                  |
    /// |-------------------------------|------------------------------------------------------------------------------------------------------------------------------|
    /// | `sender: Sender<Message>`     | This channel will be used to pass on [`messages`](mcm_misc::message::Message) to the [`console`](crate::console::Console).   |
    /// | `receiver: Receiver<Message>` | This channel will be used to receive [`messages`](mcm_misc::message::Message) from the [`console`](crate::console::Console). |
    pub fn new(config: Arc<C>, sender: Sender<Message>, receiver: Receiver<Message>) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            config,
            sender,
            receiver: Some(receiver),
            handler_list: vec![],
            handlers: HashMap::new(),
            main_thread: None,
            alive: false,
            communicator: None
        }))
    }
    pub fn set_communicator(intercom: &Arc<Mutex<InterCom<C>>>, communicator: &Arc<Mutex<Communicator<C>>>) {
        let mut intercom_lock = Self::get_lock(intercom);

        intercom_lock.communicator = Some(communicator.clone());
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
    pub fn add_handler(intercom: &Arc<Mutex<InterCom<C>>>, handler_type: char) -> Result<(String, Sender<Message>, Receiver<Message>), InterComError> {
        // check for invalid types
        match handler_type {
            'r' => {}
            'c' => {}
            _ => {
                return Err(
                    InterComError::InvalidType(handler_type)
                )
            }
        }

        let mut intercom_lock = Self::get_lock(&intercom);
        
        if let None = intercom_lock.communicator {
            log!("erro", "InterCom", "The Communicator has not yet been set.");
            return Err(InterComError::MCManageError(MCManageError::NotReady));
        }
        
        let (intercom_send, handler_receive) = mpsc::channel();
        let (handler_send, intercom_receive) = mpsc::channel();
        let handler_id: String;
        
        // add handler to handler_list
        let mut i = 0;
        loop {
            if intercom_lock.handler_list.contains(&format!("{}{}",handler_type, i)) {
                i+=1;
            }
            else {
                // valid key found
                handler_id = format!("{}{}",handler_type, i);
                // add the id to the list
                intercom_lock.handler_list.push(handler_id.clone());
                break;
            }
        }

        // add the channels to the channel storage
        if let Some(_) = intercom_lock.handlers.insert(handler_id.clone(), (handler_send, handler_receive)) {
            return Err(InterComError::DesyncedChannelStorage(handler_id))
        }
        
        Ok((handler_id, intercom_send, intercom_receive))
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
    pub fn remove_handler(intercom: &Arc<Mutex<InterCom<C>>>, handler_id: &str) -> Result<(), InterComError> {
        let mut intercom_lock = Self::get_lock_nonblocking(intercom)?;

        return Self::self_remove_handler(&mut intercom_lock, handler_id);
    }
    
    fn self_remove_handler(intercom_lock: &mut InterCom<C>, handler_id: &str) -> Result<(), InterComError> {
        // check for invalid types
        match handler_id.chars().next() {
            Some('r') => {}
            Some('c') => {}
            _ => {
                return Err(InterComError::InvalidType(handler_id.chars().next().unwrap_or(' ')))
            }
        }

        if let None = intercom_lock.communicator {
            log!("erro", "InterCom", "The Communicator has not yet been set.");
            return Err(InterComError::MCManageError(MCManageError::NotReady));
        }
        
        // remove handler from the handler_list
        let mut i = 0;
        for handler in &intercom_lock.handler_list {
            if i == intercom_lock.handler_list.len() {
                return Err(InterComError::IDNotFound(handler_id.to_string()));
            }
            else if handler == &handler_id {
                intercom_lock.handler_list.remove(i);
                break;
            }
            i+=1;
        }

        // remove channels from the channel storage
        if let None = intercom_lock.handlers.remove(&handler_id.to_string()) {
            return Err(InterComError::IDNotFound(handler_id.to_string()));
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
    fn main(intercom: Arc<Mutex<InterCom<C>>>) {
        loop {
            let mut intercom_lock;
            if let Ok(lock) = Self::get_lock_nonblocking(&intercom) {
                intercom_lock = lock
            } else {
                return;
            }

            // exit if the command got given
            if !intercom_lock.alive {
                return;
            }

            // check if the Console wants to send something and then pass it on to the right receiver if possible
            let receiver;
            if let Some(rx) = &intercom_lock.receiver {
                receiver = rx
            } else {
                log!("erro", "InterCom", "The receiver channel is missing. The Communicator will shut down.");
                Communicator::self_stop(&intercom_lock.communicator.as_ref().unwrap());

                return;
            }
            match receiver.try_recv() {
                Ok(msg) => {
                    let receiver = msg.receiver();
        
                    // get the channel to send to the receiver handler
                    if let Some(handler) = intercom_lock.handlers.get(receiver) {
                        if let Ok(_) = handler.0.send(msg) { /* message got sent */ }
                    }
                }
                Err(erro) if erro == TryRecvError::Empty => { /* There is no message currently waiting to be send */ }
                Err(_) => {
                    log!("erro", "InterCom", "The Console disconnected! The Communicator will shut down.");
                    // it is safe to unwrap here since this thread would never have started if this value was not set
                    Communicator::self_stop(&intercom_lock.communicator.as_ref().unwrap());

                    return;
                }
            }

            // check if any handler wants to send something and then pass it on to the Console
            for handler_id in &intercom_lock.handler_list.clone() {
                if let Some(rx) = intercom_lock.handlers.get(handler_id) {
                    match rx.1.try_recv() {
                        Ok(msg) => {                                        
                            // send to Console because all incoming messages have to be processed by the Console
                            if let Ok(_) = intercom_lock.sender.send(msg) { /* message got sent */ }
                        }
                        Err(erro) if erro == TryRecvError::Empty => { /* There is no message currently waiting to be send */ }
                        Err(_) => {
                            log!("erro", "InterCom", "The handler {handler_id} disconnected.");
                            if let Err(_) = Self::self_remove_handler(&mut intercom_lock, &handler_id) {
                                /* Do nothing because it must already have been removed */
                            }
                        }
                    }
                } else {
                    log!("erro", "InterCom", "The handler list did not match the handler_id list. The Communicator will be restarted.");
                    // it is safe to unwrap here since this thread would never have started if this value was not set
                    Communicator::self_restart(&intercom_lock.communicator.as_ref().unwrap());

                    return;

                }
            }

            thread::sleep(*intercom_lock.config.refresh_rate());
        }
    }
}