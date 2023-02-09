//! This module provides the [`MCServerManager struct`](MCServerManager), which is responsible for managing all [MCServers](MCServer). ( starting, stopping, ... )


use std::fs::File;
use std::io::{self, ErrorKind};
use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Instant, Duration};
use std::{fs, thread};

use serde_json::Value;

use crate::concurrent_class::ConcurrentClass;
use crate::config_trait::ConfigTrait;
use crate::{log, class_lock};
use crate::mcmanage_error::MCManageError;

use self::mcserver::MCServer;
use self::mcserver::mcserver_type::MCServerType;
use self::mcserver_manager_error::MCServerManagerError;
use self::server_list_example_default::SERVER_LIST_EXAMPLE_DEFAULT;

pub mod mcserver;
pub mod mcserver_manager_error;
pub mod server_list_example_default;
mod tests;


/// This struct is responsible for managing all [MCServers](MCServer). ( starting, stopping, ... ) \
/// In more detail, it creates [MCServer] structs accordingly to the `servers/server_list.json` file. Additionally it will also start a thread which:
///     - If set, will shut down the computer that is running this application.
///     - If enabled, will restart Minecraft servers automatically.
/// 
/// ## Warning
/// When specifying a ram limit like `-Xmx=4G` in the `servers/server_list.json` file, the Minecraft server can fail to start.
/// 
/// ## Methods
/// | Method                                                              | Description                                                                                                                 |
/// |---------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------|
/// | [`new(...) -> Result<...>`](MCServerManager::new)                   | Create a new [`MCServerManager`] instance and create all MCServer structs according to the `servers/server_list.json` file. |
/// | [`get_mcserver(...) -> Result<...>`](MCServerManager::get_mcserver) | Search for a [`MCServer`] by its name and return it if found.                                                               |
/// 
/// ... and other methods inherited by the [`ConcurrentClass trait`](ConcurrentClass).
pub struct MCServerManager<C: ConfigTrait> {
    /// A list of every [`MCServer`]
    mcserver_list: Vec<Arc<Mutex<MCServer<C>>>>,
    /// The application's config
    config: Arc<C>,
    /// The main thread of this application
    main_thread: Option<thread::JoinHandle<()>>,
    /// Controls whether or not the main thread should run
    alive: bool
}
impl<C: ConfigTrait> ConcurrentClass<MCServerManager<C>, C> for MCServerManager<C> {
    fn get_config_unlocked(class_lock: &MutexGuard<MCServerManager<C>>) -> Arc<C> {
        class_lock.config.clone()
    }
    fn get_name_unlocked(_: &MutexGuard<MCServerManager<C>>) -> String {
        "MCServerManager".to_string()
    }
    fn get_name_poison_error(_: &MutexGuard<MCServerManager<C>>) -> String {
        "MCServerManager".to_string()
    }
    fn get_default_state(class_lock: &mut MutexGuard<MCServerManager<C>>) -> MCServerManager<C> {
        let mut mcserver_manager = MCServerManager {
            mcserver_list: vec![],
            config: class_lock.config.clone(),
            main_thread: None,
            alive: false
        };

        if let Err(erro) = Self::load_mcserver_list_unlocked(&mut mcserver_manager) {
            log!("erro", "MCServerManager", "Found this error while loading the `servers/server_list.json` file. Error: {erro}");

            unimplemented!("Handle this error inside `MCServerManager::get_default_state() // Self::load_mcserver_list_unlocked()` Error: {erro}");
        }

        return mcserver_manager;
    }
    fn start(class: &Arc<Mutex<MCServerManager<C>>>, log_messages: bool) -> Result<(), MCManageError> {
        let mut class_lock = class_lock!(class, log_messages);

        for mcserver in &class_lock.mcserver_list {
            MCServer::self_start(mcserver);
        }

        class_lock.alive = true;

        let class_clone = class.clone();
        class_lock.main_thread = Some(thread::spawn(move || {
            Self::main(&class_clone);
        }));

        Ok(())
    }
    fn stop(class: &Arc<Mutex<MCServerManager<C>>>, log_messages: bool) -> Result<(), MCManageError> {
        let mut class_lock = class_lock!(class, log_messages);

        for mcserver in &class_lock.mcserver_list {
            MCServer::self_stop(mcserver);
        }

        class_lock.alive = false;

        // acquire the main thread
        let main_thread;
        if let Some(main) = class_lock.main_thread.take() {
            main_thread = main;
        } else {
            if log_messages { log!("erro", "MCServerManager", "Could not take the main thread. It was already taken."); }
            Self::reset_unlocked(&mut class_lock);
            return Err(MCManageError::FatalError);
        }

        drop(class_lock);

        // wait for the main thread to finish
        if let Err(_) = main_thread.join() {
            if log_messages { log!("erro", "MCServerManager", "Failed to join the main thread."); }
            Self::reset(class);
            return Err(MCManageError::FatalError);
        }

        Ok(())   
    }
}
impl<C: ConfigTrait> MCServerManager<C> {
    /// Create a new [`MCServerManager`] instance and create all MCServer structs according to the `servers/server_list.json` file.
    pub fn new(config: Arc<C>) -> Result<Arc<Mutex<Self>>, MCServerManagerError> {
        let mcserver_manager = Arc::new(Mutex::new(Self {
            mcserver_list: vec![],
            config,
            main_thread: None,
            alive: false
        }));
        
        Self::load_mcserver_list(&mcserver_manager)?;
        
        return Ok(mcserver_manager);
    }

    /// Create the MCServers according to the `servers/server_list.json` file. \
    /// If any problem is detected in the `servers/server_list.json` file, this file will be renamed to `servers/invalid_server_list.json` and an example file will be
    /// generated under `servers/server_list_example.json`. \
    /// \
    /// This function requires the [`MCServerManager`] to be locked. Use the [`load_mcserver_list_unlocked function`](MCServerManager::load_mcserver_list_unlocked) for a 
    /// unlocked [`MCServerManager`].
    /// 
    /// ## Warning
    /// When specifying a ram limit like `-Xmx=4G` in the `servers/server_list.json` file, the Minecraft server can fail to start.
    fn load_mcserver_list(mcserver_manager: &Arc<Mutex<MCServerManager<C>>>) -> Result<(), MCServerManagerError> {
        let mut mcserver_manager_lock = Self::get_lock(&mcserver_manager);
        return Self::load_mcserver_list_unlocked(&mut mcserver_manager_lock);
    }
    /// Create the MCServers according to the `servers/server_list.json` file. \
    /// If any problem is detected in the `servers/server_list.json` file, this file will be renamed to `servers/invalid_server_list.json` and an example file will be
    /// generated under `servers/server_list_example.json`. \
    /// \
    /// This function requires the [`MCServerManager`] to be unlocked. Use the [`load_mcserver_list function`](MCServerManager::load_mcserver_list) for a locked one.
    /// 
    /// ## Warning
    /// When specifying a ram limit like `-Xmx=4G` in the `servers/server_list.json` file, the Minecraft server can fail to start.
    /// [`MCServerManager`].
    fn load_mcserver_list_unlocked(mcserver_manager_lock: &mut MCServerManager<C>) -> Result<(), MCServerManagerError> {
        // read the 'servers/server_list.json' file to a json object
        let mcserver_list_json: Value;
        match fs::read_to_string("servers/server_list.json") {
            Ok(file) => {
                if let Ok(json) = serde_json::from_str(&file) {
                    mcserver_list_json = json;
                } else {
                    log!("erro", "MCServerManager", "{}", MCServerManagerError::InvalidFile);
                    Self::generate_valid_server_list_file();
                    return Err(MCServerManagerError::InvalidFile);
                }
            }
            Err(erro) => {
                if let ErrorKind::NotFound = erro.kind() {
                    if Path::new("servers/server_list_example.json").exists() {
                        log!("erro", "MCServerManager", "To start any MCServer, you need to configure it in the 'servers/server_list.json' file.");
                        log!("erro", "MCServerManager", "See the 'servers/server_list_example.json' file for a valid write style.");
                        return Err(MCServerManagerError::IOError(erro));
                    } else {
                        log!("erro", "MCServerManager", "The 'servers/server_list.json' file could not be found. A valid example will be generated under 'servers/server_list_example.json'.");
                    }
                } else {
                    log!("erro", "MCServerManager", "An error occurred while opening the 'servers/server_list.json' file. A valid example will be generated under 'servers/server_list_example.json'.");
                }
                Self::generate_valid_server_list_file();
                return Err(MCServerManagerError::IOError(erro));
            }
        }


        // create a list of MCServers and return it
        let mut mcserver_list: Vec<Arc<Mutex<MCServer<C>>>> = vec![];
        let mut i = 0;
        loop {
            if let Some(server) = mcserver_list_json.get(i.to_string()) {
                let name = &Self::get_server_parameter(server, i, "name")?;
                let arg = &Self::get_server_parameter(server, i, "arg")?;
                let mcserver_type = &Self::get_server_parameter(server, i, "type")?;

                mcserver_list.push(MCServer::new(name, arg, MCServerType::new(mcserver_type), &mcserver_manager_lock.config.clone()));
            } else {
                if i == 0 {
                    log!("erro", "MCServerManager", "The 'servers/server_list.json' file did not contain any servers. See the example file for a valid style.");
                    Self::generate_valid_server_list_file();
                    return Err(MCServerManagerError::InvalidFile);
                }
                mcserver_manager_lock.mcserver_list = mcserver_list;
                return Ok(());
            }
            i+=1;
        }
    }
    /// Read a given parameter of a json object and return its value in the form of a string.
    fn get_server_parameter(server_json: &Value, server_id: i32, parameter_name: &str) -> Result<String, MCServerManagerError> {
        if let Some(value) = server_json.get(parameter_name) {
            if let Some(real_value) = value.as_str() {
                return Ok(real_value.to_string());
            } else {
                log!("erro", "MCServerManager", "The '{parameter_name}' parameter of server {server_id} should be a string. See the 'servers/server_list_example.json' file for a valid write style.");
            }
        } else {
            log!("erro", "MCServerManager", "The server {server_id} is missing a '{parameter_name}' parameter. See the 'servers/server_list_example.json' file for a valid write style."); 
        }
        Self::generate_valid_server_list_file();
        return Err(MCServerManagerError::InvalidFile);
    }
    /// Rename the current `servers/server_list.json` file to `servers/invalid_server_list.json` and generate an example file under `servers/server_list_example.json`.
    fn generate_valid_server_list_file() {
        // rename the invalid file, if available, so that data will not get lost
        let mut invalid_file_name;
        let mut i = 0;
        loop {
            if i == 0 {
                invalid_file_name = format!("servers/invalid_server_list.json");
            } else {
                invalid_file_name = format!("servers/invalid_server_list({}).json", i);
            }
            if !Path::new(&invalid_file_name).exists() {
                if let Err(_) = fs::rename("servers/server_list.json", &invalid_file_name) {
                    // the file does not exist -> the folder probably also not

                    if let Err(erro) = fs::create_dir("servers") {
                        match erro.kind() {
                            ErrorKind::AlreadyExists => {}
                            _ => { panic!("This error occurred while trying to create the servers folder: {erro}") }
                        }
                    }
                }
                break;
            } else {
                i += 1;
            }
        }

        // generate the valid file
        let mut server_list_example_file = File::options().write(true).create(true).open("servers/server_list_example.json").unwrap(); // no error is expected, so we unwrap here
        io::copy(&mut SERVER_LIST_EXAMPLE_DEFAULT.as_bytes(), &mut server_list_example_file).unwrap(); // no error is expected, so we unwrap here
    }

    /// Search for a [`MCServer`] by its name and return it if found.
    pub fn get_mcserver(mcserver_manager: &Arc<Mutex<MCServerManager<C>>>, mcserver_name: &str) -> Result<Arc<Mutex<MCServer<C>>>, MCServerManagerError> {
        let mcserver_manager_lock = MCServerManager::get_lock_nonblocking(mcserver_manager)?;

        for mcserver in &mcserver_manager_lock.mcserver_list {
            if MCServer::get_name_unlocked(&MCServer::get_lock(&mcserver)) == mcserver_name {
                return Ok(mcserver.clone());
            }
        }

        return Err(MCServerManagerError::NotFound)
    }

    /// This main loop will do multiple things:
    ///     - the computer running this application will shut down if configured
    ///     - the Minecraft servers will restart automatically if configured
    fn main(mcserver_manager: &Arc<Mutex<MCServerManager<C>>>) {
        log!("", "MCServerManager", "Started!");

        let mut offline_counter: Option<Instant> = None;
        let mut last_restart = Instant::now();
        loop {
            let mcserver_manager_lock;
            if let Ok(lock) = Self::get_lock_nonblocking(mcserver_manager) {
                mcserver_manager_lock = lock;
            } else {
                return;
            }

            if !mcserver_manager_lock.alive {
                return;
            }


            // check if any player is online
            let mut player_online = false;
            for mcserver in &mcserver_manager_lock.mcserver_list {
                let player_list;
                if let Ok(list) = MCServer::get_players(mcserver) {
                    player_list = list;
                } else {
                    continue;
                }

                if player_list.len() > 0 {
                    player_online = true;
                    break;
                }
            }

            // shut down the computer running this application if configured
            if *mcserver_manager_lock.config.shutdown_time() > Duration::new(0, 0) {
                if let Some(offline_counter) = offline_counter {
                    if Instant::now() - offline_counter >= *mcserver_manager_lock.config.shutdown_time() {
                        log!("", "MCServerManager", "No player was active for {:?}. This machine will now shut down.", mcserver_manager_lock.config.shutdown_time());
                        system_shutdown::shutdown().unwrap();
                    }
                }

                if player_online {
                    offline_counter = None;
                } else {
                    if let None = offline_counter {
                        offline_counter = Some(Instant::now());
                    }
                }
            }

            // restart the MCServers automatically every configured amount of time
            if *mcserver_manager_lock.config.mcserver_restart_time() > Duration::new(0, 0) {
                if Instant::now() - last_restart >= *mcserver_manager_lock.config.mcserver_restart_time() {
                    log!("", "MCServerManager", "The automatic restart time of {:?} has been reached. All MCServer's will now restart.", *mcserver_manager_lock.config.mcserver_restart_time());
                    for mcserver in &mcserver_manager_lock.mcserver_list {
                        MCServer::self_restart(mcserver);
                    }
                    last_restart = Instant::now();
                }
            }


            let refresh_rate = *mcserver_manager_lock.config.refresh_rate();
            drop(mcserver_manager_lock);
            thread::sleep(refresh_rate);
        }
    }
}