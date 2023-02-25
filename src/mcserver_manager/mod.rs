//! This module provides the [`MCServerManager`](MCServerManager) struct, which is responsible for managing all [`MCServers`](MCServer). ( starting, stopping, ... )


use std::fs::{self, File};
use std::io::{ErrorKind, self};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread::{self, JoinHandle};
use std::time::{Instant, Duration};
use std::sync::atomic::Ordering::Relaxed;

use async_trait::async_trait;
use serde_json::Value;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use tokio::sync::oneshot::{Sender, channel};
use tokio::time::sleep;

use crate::concurrent_class::ConcurrentClass;
use crate::concurrent_class::qol_functions::{check_allowed_start, check_allowed_stop};
use crate::concurrent_class::status::Status;
use crate::config::Config;
use crate::log;
use crate::mcmanage_error::MCManageError;

use self::mcserver::MCServer;
use self::mcserver::mcserver_type::MCServerType;
use self::server_list_example_default::SERVER_LIST_EXAMPLE_DEFAULT;


pub mod mcserver;
pub mod server_list_example_default;
mod tests;


/// This struct is responsible for managing all [`MCServers`](MCServer). ( starting, stopping, ... ) \
/// In more detail, it creates [`MCServer`] structs accordingly to the `servers/server_list.json` file. Additionally it will also start a thread which:
///     - If set, will shut down the computer that is running this application.
///     - If enabled, will restart Minecraft servers automatically.
/// 
/// # Warning
/// When specifying a ram limit like `-Xmx=4G` in the `servers/server_list.json` file, the Minecraft server can fail to start.
/// 
/// # Methods
/// | Method                                                              | Description                                                                                                                 |
/// |---------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------|
/// | [`new(...) -> Result<...>`](MCServerManager::new)                   | Create a new [`MCServerManager`] instance and create all MCServer structs according to the `servers/server_list.json` file. |
/// | [`get_all(...) -> Result<...>`](MCServerManager::get_all)           | Return a list of every [`MCServer`].                                                                                        |
/// | [`get_mcserver(...) -> Result<...>`](MCServerManager::get_mcserver) | Search for a [`MCServer`] by its name and return it if found.                                                               |
/// 
/// ... and other methods inherited by the [`ConcurrentClass`](ConcurrentClass) trait.
pub struct MCServerManager {
    name: String,
    config: Arc<Config>,

    alive: AtomicBool,
    status: Mutex<Status>,
    mcserver_list: Mutex<Vec<Arc<MCServer>>>,
    main_thread: Mutex<Option<JoinHandle<()>>>
}
#[async_trait]
impl ConcurrentClass for MCServerManager {
    fn name(self: &Arc<Self>) -> String {
        self.name.clone()
    }
    fn config(self: &Arc<Self>) -> Arc<Config> {
        self.config.clone()
    }
    async fn status(self: &Arc<Self>) -> Status {
        self.status.lock().await.clone()
    }
    async fn set_status(self: &Arc<Self>, new_status: Status) {
        *self.status.lock().await = new_status
    }
    async fn reset(self: &Arc<Self>) {
        self.alive.store(false, Relaxed);
        *self.status.lock().await = Status::Stopped;
        *self.mcserver_list.lock().await = vec![];
        *self.main_thread.lock().await = None;
    }
    async fn impl_start(self: Arc<Self>, restart: bool) -> Result<(), MCManageError> {
        check_allowed_start(&self, restart).await?;
        
        let start_time = Instant::now();        
        if !restart { log!("", self.name, "Starting..."); }

        self.load_mcserver_list().await?;
        
        for mcserver in &*self.mcserver_list.lock().await {
            mcserver.start();
        }

        let (tx, _) = channel();

        self.alive.store(true, Relaxed);
        let mcserver_manager = self.clone();
        *self.main_thread.lock().await = Some(thread::spawn(move || {
            let runtime = Runtime::new().unwrap();
            if let Err(_) = runtime.block_on(mcserver_manager.clone().main(Some(tx))) {}
        }));

        *self.status.lock().await = Status::Started;

        if !restart { log!("", self.name, "Started in {:.3} secs!", start_time.elapsed().as_secs_f64()); }

        Ok(())
    }
    async fn impl_stop(self: Arc<Self>, restart: bool, forced: bool) -> Result<(), MCManageError> {
        check_allowed_stop(&self, restart, forced).await?;
        
        let stop_time = Instant::now();
        
        // wait for the MCServers to finish
        for mcserver in &*self.mcserver_list.lock().await {
            if let Err(_) = mcserver.clone().impl_stop(false, true).await {}
        }


        if !restart { log!("", self.name, "Stopping..."); }

        // wait for the main thread to finish
        self.alive.store(false, Relaxed);
        if let Some(thread) = self.main_thread.lock().await.take() {
            if let Err(_) = thread.join() {
                log!("erro", self.name, "Failed to join the main thread.");
                self.reset().await;
                return Err(MCManageError::FatalError);
            }
        } else {
            log!("erro", self.name, "Could not take the main thread. It was already taken.");
            self.reset().await;
            return Err(MCManageError::FatalError);
        }

        if !restart { log!("", self.name, "Stopped in {:.3} secs!", stop_time.elapsed().as_secs_f64()); }

        Ok(())
    }
    async fn main(self: Arc<Self>, _: Option<Sender<()>>) -> Result<(), MCManageError> {
        let mut offline_counter: Option<Instant> = None;
        let mut last_restart = Instant::now();

        while self.alive.load(Relaxed) {
            // check if any player is online
            let mut player_online = false;
            for mcserver in &*self.mcserver_list.lock().await {
                let player_list = mcserver.players().await;

                if player_list.len() > 0 {
                    player_online = true;
                    break;
                }
            }

            // shut down the computer running this application if configured
            if *self.config.shutdown_time() > Duration::new(0, 0) {
                if let Some(offline_counter) = offline_counter {
                    if Instant::now() - offline_counter >= *self.config.shutdown_time() {
                        log!("", self.name, "No player was active for {:?}. This machine will now shut down.", self.config.shutdown_time());
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
            if *self.config.mcserver_restart_time() > Duration::new(0, 0) {
                if Instant::now() - last_restart >= *self.config.mcserver_restart_time() {
                    log!("", self.name, "The automatic restart time of {:?} has been reached. All MCServer's will now restart.", *self.config.mcserver_restart_time());
                    for mcserver in &*self.mcserver_list.lock().await {
                        mcserver.restart();
                    }
                    last_restart = Instant::now();
                }
            }

            sleep(*self.config.refresh_rate()).await;
        }
        Ok(())
    }
}
impl MCServerManager {
    /// Create a new [`MCServerManager`] instance.
    pub fn new(config: Arc<Config>) -> Arc<Self> {
        Arc::new(Self {
            name: "MCServerManager".to_string(),
            config,

            alive: AtomicBool::new(false),
            status: Status::Stopped.into(),
            mcserver_list: vec![].into(),
            main_thread: None.into()
        })
    }
    /// Create the MCServers according to the `servers/server_list.json` file. \
    /// If any problem is detected in the `servers/server_list.json` file, this file will be renamed to `servers/invalid_server_list.json` and an example file will be
    /// generated under `servers/server_list_example.json`.
    /// 
    /// # Warning
    /// When specifying a ram limit like `-Xmx=4G` in the `servers/server_list.json` file, the Minecraft server can fail to start.
    async fn load_mcserver_list(self: &Arc<Self>) -> Result<(), MCManageError> {
        // read the 'servers/server_list.json' file to a json object
        let mcserver_list_json: Value;
        match fs::read_to_string("servers/server_list.json") {
            Ok(file) => {
                if let Ok(json) = serde_json::from_str(&file) {
                    mcserver_list_json = json;
                } else {
                    log!("erro", self.name, "{}", MCManageError::InvalidFile);
                    self.generate_valid_server_list_file();
                    return Err(MCManageError::InvalidFile);
                }
            }
            Err(erro) => {
                if let ErrorKind::NotFound = erro.kind() {
                    if Path::new("servers/server_list_example.json").exists() {
                        log!("erro", self.name, "To start any MCServer, you need to configure it in the 'servers/server_list.json' file.");
                        log!("erro", self.name, "See the 'servers/server_list_example.json' file for a valid write style.");
                        return Err(MCManageError::IOError(erro));
                    } else {
                        log!("erro", self.name, "The 'servers/server_list.json' file could not be found. A valid example will be generated under 'servers/server_list_example.json'.");
                    }
                } else {
                    log!("erro", self.name, "An error occurred while opening the 'servers/server_list.json' file. A valid example will be generated under 'servers/server_list_example.json'.");
                }
                self.generate_valid_server_list_file();
                return Err(MCManageError::IOError(erro));
            }
        }


        // create a list of MCServers and return it
        let mut mcserver_list: Vec<Arc<MCServer>> = vec![];
        let mut i = 0;
        loop {
            if let Some(server) = mcserver_list_json.get(i.to_string()) {
                let name = &self.get_server_parameter(server, i, "name")?;
                let arg = &self.get_server_parameter(server, i, "arg")?;
                let mcserver_type = &self.get_server_parameter(server, i, "type")?;

                mcserver_list.push(MCServer::new(name, arg, MCServerType::new(mcserver_type, name), &self.config.clone()));
            } else {
                if i == 0 {
                    log!("erro", "MCServerManager", "The 'servers/server_list.json' file did not contain any servers. See the example file for a valid style.");
                    self.generate_valid_server_list_file();
                    return Err(MCManageError::InvalidFile);
                }
                *self.mcserver_list.lock().await = mcserver_list;
                return Ok(());
            }
            i+=1;
        }
    }
    /// Read a given parameter of a json object and return its value in the form of a string.
    fn get_server_parameter(self: &Arc<Self>, server_json: &Value, server_id: i32, parameter_name: &str) -> Result<String, MCManageError> {
        if let Some(value) = server_json.get(parameter_name) {
            if let Some(real_value) = value.as_str() {
                return Ok(real_value.to_string());
            } else {
                log!("erro", self.name, "The '{parameter_name}' parameter of server {server_id} should be a string. See the 'servers/server_list_example.json' file for a valid write style.");
            }
        } else {
            log!("erro", self.name, "The server {server_id} is missing a '{parameter_name}' parameter. See the 'servers/server_list_example.json' file for a valid write style."); 
        }
        self.generate_valid_server_list_file();
        return Err(MCManageError::InvalidFile);
    }
    /// Rename the current `servers/server_list.json` file to `servers/invalid_server_list.json` and generate an example file under `servers/server_list_example.json`.
    fn generate_valid_server_list_file(self: &Arc<Self>) {
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
    /// Return a list of every [`MCServer`].
    pub async fn get_all(self: &Arc<Self>) -> Result<Vec<Arc<MCServer>>, MCManageError> {
        return Ok(self.mcserver_list.lock().await.clone())
    }
    /// Search for a [`MCServer`] by its name and return it if found.
    pub async fn get_mcserver(self: &Arc<Self>, mcserver_name: &str) -> Result<Arc<MCServer>, MCManageError> {
        for mcserver in &*self.mcserver_list.lock().await {
            if mcserver.name() == mcserver_name {
                return Ok(mcserver.clone());
            }
        }

        return Err(MCManageError::NotFound)
    }
}