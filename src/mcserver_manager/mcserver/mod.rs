//! This module provides the [`MCServer struct`](MCServer) which represents an API for one Minecraft server, which got assigned with the initiation of this struct.


use std::fs::{File, self};
use std::io::{BufReader, BufRead, Write, Read, ErrorKind};
use std::path::Path;
use std::process::{Command, Child, Stdio, ChildStdout};
use std::sync::{Mutex, Arc, MutexGuard};
use std::thread;
use std::time::Instant;

use crate::concurrent_class::ConcurrentClass;
use crate::log;
use crate::config_trait::ConfigTrait;
use crate::mcmanage_error::MCManageError;
use self::mcserver_error::MCServerError;
use self::mcserver_type::MCServerType;
use self::mcserver_status::MCServerStatus;


mod tests;
pub mod mcserver_error;
pub mod mcserver_type;
pub mod mcserver_status;


/// This struct represents an API for one Minecraft server, which got assigned with the initiation of this struct. \
/// 
/// 
/// ## Features
/// 
/// - The log of the Minecraft server running gets saved to ' logs/{MCServer.name}.txt '.
/// - Lines of text can be sent to the Minecraft server.
/// - The names of the players currently on the Minecraft server get saved.
/// - The [`status`](mcserver_status::MCServerStatus) of the Minecraft server gets saved. ( Starting, Stopping, ... )
/// 
/// 
/// ## Methods
/// 
/// | Method                                                              | Description                                              |
/// |---------------------------------------------------------------------|----------------------------------------------------------|
/// | [`new(...) -> Arc<Mutex<MCServer>>`](MCServer::new)                 | Create a new [`MCServer`] instance.                      |
/// | [`get_status(...) -> Result<..>`](MCServer::get_status)             | Get the status of the [`MCServer`].                      |
/// | [`get_players(...) -> Result<..>`](MCServer::get_players)           | Get the list of players of the [`MCServer`].             |
/// | [`start(...) -> Result<..>`](MCServer::start)                       | Start the [`MCServer`].                                  |
/// | [`stop(...) -> Result<..>`](MCServer::stop)                         | Stop the [`MCServer`].                                   |
/// | [`restart(...) -> Result<..>`](MCServer::restart)                   | Restart the [`MCServer`].                                |
/// | [`send_input(...)`](MCServer::send_input)                           | Send a given string to the Minecraft server as an input. |
pub struct MCServer<C: ConfigTrait> {
    /// The name of this MCServer.
    name: String,
    /// The arguments for starting the Minecraft server. ( for example: '-jar purpur-1.19.3-1876.jar -Xmx4G nogui' )
    arg: Vec<String>,
    /// The path to the Minecraft server.
    path: String,
    /// The type of the Minecraft server. ( vanilla, purpur, ... )
    mcserver_type: Arc<Mutex<MCServerType>>,
    /// The application's config.
    config: Arc<C>,

    /// The Minecraft server process.
    minecraft_server: Option<Child>,
    /// The main thread of this struct.
    main_thread: Option<thread::JoinHandle<()>>,
    
    /// Controls whether or not this MCServer should run.
    alive: bool,
    /// The [`status`](mcserver_status::MCServerStatus) of the Minecraft server. ( Starting, Stopping, ... )
    status: MCServerStatus,
    /// The list of players on the Minecraft server.
    players: Vec<String>
}
impl<C: ConfigTrait> ConcurrentClass<MCServer<C>, C> for MCServer<C> {
    fn get_config_unlocked(class_lock: &MutexGuard<MCServer<C>>) -> Arc<C> {
        return class_lock.config.clone();
    }
    fn get_name_unlocked(class_lock: &MutexGuard<MCServer<C>>) -> String {
        return class_lock.name.clone();
    }
    fn get_name_poison_error(class_lock: &MutexGuard<MCServer<C>>) -> String {
        return class_lock.name.clone();
    }
    fn get_default_state(class_lock: &mut MutexGuard<MCServer<C>>) -> MCServer<C> {
        Self {
            name: class_lock.name.clone(),
            arg: class_lock.arg.clone(),
            path: format!("servers/{}", class_lock.name),
            mcserver_type: class_lock.mcserver_type.clone(),
            config: class_lock.config.clone(),

            minecraft_server: None,
            main_thread: None,
            
            alive: false,
            status: MCServerStatus::Stopped,
            players: vec![],
        }
    }
    fn start(mcserver: &Arc<Mutex<MCServer<C>>>, log_messages: bool) -> Result<(), MCManageError> {
        let mut mcserver_lock = Self::get_lock(mcserver);
        let mcserver_clone = mcserver.clone();
        
        let name = mcserver_lock.name.clone();

        mcserver_lock.status = MCServerStatus::Starting;

        match Command::new("java")
            .current_dir(&mcserver_lock.path)
            .args(&mcserver_lock.arg)
            .stderr(Stdio::piped())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Ok(minecraft_server) => {
                mcserver_lock.minecraft_server = Some(minecraft_server);
            }
            Err(err) => {
                if log_messages { log!("erro", &name, "An error occurred while starting the Minecraft Server {name}. Error: {err}"); }
                Self::reset_unlocked(&mut mcserver_lock);
                return Err(MCManageError::FatalError)
            }
        }
        
        mcserver_lock.alive = true;

        mcserver_lock.main_thread = Some(thread::spawn(move ||
            Self::main(mcserver_clone, log_messages)
        ));

        return Ok(());
    }
    fn stop(mcserver: &Arc<Mutex<MCServer<C>>>, log_messages: bool) -> Result<(), MCManageError> {
        let mut mcserver_lock;
        if let Some(lock) = Self::get_lock_pure(mcserver, false) {
            mcserver_lock = lock;
        } else {
            if log_messages { log!("erro", "MCServer", "A MCServer got corrupted."); }
            MCServer::reset(&mcserver);
            return Err(MCManageError::FatalError);
        }


        let name = mcserver_lock.name.clone();
        let stop_time = Instant::now();

        // check if the MCServer has started
        if !log_messages {
        } else if mcserver_lock.status != MCServerStatus::Started {
            return Err(MCManageError::NotReady);
        } else if mcserver_lock.status == MCServerStatus::Stopped {
            // do nothing since this server is already stopped
            return Ok(());
        }
        

        if log_messages { log!("info", &name, "Stopping..."); }

        mcserver_lock.status = MCServerStatus::Stopping;

        if let Some(mut minecraft_server ) = mcserver_lock.minecraft_server.take() {
            // send the stop command to the Minecraft server
            if let Some(stdin) = minecraft_server.stdin.as_mut() {
                if let Err(err) = stdin.write_all(format!("stop\n").as_bytes()) {
                    if log_messages { log!("erro", &name, "An error occurred while writing the input `stop` to the Minecraft server. The process will be kill forcefully. Error: {err}"); }
                    if let Err(_) = minecraft_server.kill() {}
                }
            } else {
                if log_messages { log!("erro", &name, "The stdin pipe of this Minecraft server process does not exist. The process will be kill forcefully."); }
                if let Err(_) = minecraft_server.kill() {}
            }

            // wait for the Minecraft server to finish
            if let Err(err) = minecraft_server.wait() {
                if log_messages { log!("erro", &name, "An error occurred while waiting on the Minecraft server to finish. Error: {err}"); }
                Self::reset_unlocked(&mut mcserver_lock);
                return Err(MCManageError::FatalError);
            }
        } else {
            if log_messages { log!("erro", &name, "Could not get the Minecraft server. It was already taken."); }
            Self::reset_unlocked(&mut mcserver_lock);
            return Err(MCManageError::FatalError);
        }

        // give the shutdown command
        mcserver_lock.alive = false;
        
        // acquire the main thread
        let main_thread;
        if let Some(main) = mcserver_lock.main_thread.take() {
            main_thread = main;
        } else {
            if log_messages { log!("erro", &name, "Could not take the main thread. It was already taken."); }
            Self::reset_unlocked(&mut mcserver_lock);
            return Err(MCManageError::FatalError);
        }

        drop(mcserver_lock);

        // wait for the main thread to finish
        if let Err(_) = main_thread.join() {
            if log_messages { log!("erro", "MCServer", "Failed to join the main thread."); }
            MCServer::reset(&mcserver);
            return Err(MCManageError::FatalError);
        }

        // set the MCServers status to stopped
        if let Ok(mut mcserver_lock) = mcserver.lock() {
            mcserver_lock.status = MCServerStatus::Stopped;
        } else {
            if log_messages { log!("erro", &name, "This MCServer got corrupted."); }
            MCServer::reset(&mcserver);
            return Err(MCManageError::FatalError);
        }

        if log_messages { log!("info", &name, "Stopped in {:.3} secs!", stop_time.elapsed().as_secs_f64()); }

        Ok(())
    }
    fn wait_for_start_confirm(mcserver: &Arc<Mutex<MCServer<C>>>) {
        loop {
            let mcserver_lock;
            if let Ok(lock) = Self::get_lock_nonblocking(mcserver) {
                mcserver_lock = lock;
            } else {
                return;
            }

            if let MCServerStatus::Started = mcserver_lock.status {
                return;
            } else {
                thread::sleep(*mcserver_lock.config.refresh_rate());
            }
        }
    }
}
impl<C: ConfigTrait> MCServer<C> {
    /// Create a new [`MCServer`] instance.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                     | Description                                                                                                  |
    /// |-------------------------------|--------------------------------------------------------------------------------------------------------------|
    /// | `name: &str`                  | The name of the Minecraft server.                                                                            |
    /// | `arg: &str`                   | The arguments for starting the Minecraft server. ( for example: '-jar purpur-1.19.3-1876.jar -Xmx4G nogui' ) |
    /// | `path: &str`                  | The path to the Minecraft server's location.                                                                 |
    /// | `mcserver_type: MCServerType` | The type of the Minecraft server. ( vanilla, purpur, ... )                                                   |
    /// | `config: &Arc<C>`             | The application's config.                                                                                    |
    pub fn new(name: &str, arg: &str, mcserver_type: MCServerType, config: &Arc<C>) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self {
            name: name.to_owned(),
            arg: arg.split(" ").map(String::from).collect(),
            path: format!("servers/{}", name),
            mcserver_type: Arc::new(Mutex::new(mcserver_type)),
            config: config.clone(),

            minecraft_server: None,
            main_thread: None,
            
            alive: false,
            status: MCServerStatus::Stopped,
            players: vec![],
        }))
    }

    /// Get the status of the [`MCServer`].
    pub fn get_status(mcserver: &Arc<Mutex<MCServer<C>>>) -> Result<MCServerStatus, MCServerError> {
        let mcserver_lock;
        if let Some(lock) = Self::get_lock_pure(mcserver, true) {
            mcserver_lock = lock;
        } else {
            Self::self_restart(mcserver);
            return Err(MCServerError::CriticalError);
        }

        return Ok(mcserver_lock.status.clone());
    }
    /// Get the list of players of the [`MCServer`].
    pub fn get_players(mcserver: &Arc<Mutex<MCServer<C>>>) -> Result<Vec<String>, MCServerError> {
        let mcserver_lock;
        if let Some(lock) = Self::get_lock_pure(mcserver, true) {
            mcserver_lock = lock;
        } else {
            Self::self_restart(mcserver);
            return Err(MCServerError::CriticalError);
        }

        return Ok(mcserver_lock.players.clone());
    }

    /// Get the [`mcserver type`](mcserver_type::MCServerType) of a given MCServer.
    fn get_mcserver_type(mcserver_lock: &MutexGuard<MCServer<C>>, mcserver: &Arc<Mutex<MCServer<C>>>) -> Result<MCServerType, MCServerError> {
        if let Ok(mcserver_type) = mcserver_lock.mcserver_type.lock() {
            return Ok((*mcserver_type).clone());
        } else {
            log!("erro", mcserver_lock.name, "The mcserver_type got corrupted. This MCServer will be restarted.");
            Self::self_restart(mcserver);
            return Err(MCServerError::CriticalError);
        }
    }

    /// Send a given string to the Minecraft server as an input. \
    /// It is guaranteed that the string given will be sent to the MCServer, but this can cause the blocking of the thread calling this function due to the MCServer restarting.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                         | Description                                                             |
    /// |-----------------------------------|-------------------------------------------------------------------------|
    /// | `mcserver: &Arc<Mutex<MCServer>>` | A reference to the MCServer struct which started this Minecraft server. |
    /// | `input: &str`                     | The string to send to the Minecraft server.                             |
    pub fn send_input(mcserver: &Arc<Mutex<MCServer<C>>>, input: &str) {
        let mut mcserver_lock = Self::get_lock(mcserver);
        
        if let Some(child ) = mcserver_lock.minecraft_server.as_mut() {
            if let Some(stdin) = child.stdin.as_mut() {
                if let Err(err) = stdin.write_all(format!("{input}\n").as_bytes()) {
                    log!("erro", mcserver_lock.name, "An error occurred while writing the input `{input}` to the Minecraft server. This MCServer will be restarted. Error: {err}");
                    loop {
                        if let Err(erro) = Self::restart(mcserver) {
                            if let MCManageError::NotReady = erro {
                                thread::sleep(*mcserver_lock.config.refresh_rate());
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    Self::send_input(mcserver, input);
                }
            } else {
                log!("erro", mcserver_lock.name, "The stdin pipe of this Minecraft server process does not exist. This MCServer will be restarted.");
                loop {
                    if let Err(erro) = Self::restart(mcserver) {
                        if let MCManageError::NotReady = erro {
                            thread::sleep(*mcserver_lock.config.refresh_rate());
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                Self::send_input(mcserver, input);
            }
        } else {
            log!("erro", mcserver_lock.name, "The Minecraft server process could not be found. Please start the Minecraft server before sending input to it.");
            return; // there is no need to properly stop the Minecraft server because none is running
        }
    }

    /// This represents the main thread of this MCServer struct. \
    /// It is responsible for the reading and interpreting part. Through this function, the application is able to understand what is happening on this server and what it
    /// should do next. 
    fn main(mcserver: Arc<Mutex<MCServer<C>>>, log_messages: bool) {        
        let mut started = false;
        let start_time = Instant::now();
        let mut agreed_to_eula = false;
        let mut mcserver_lock;
        if let Ok(lock) = Self::get_lock_nonblocking(&mcserver) {
            mcserver_lock = lock;
        } else {
            // the MCServer got corrupted and is now restarting
            return;
        }

        if log_messages {
            log!("info", mcserver_lock.name, "Starting...");
        }
        
        let stdout;
        if let Ok(result) = Self::get_stdout_pipe(&mut mcserver_lock) {
            stdout = BufReader::new(result)
        } else {
            Self::self_restart(&mcserver);
            return;
        }

        drop(mcserver_lock);

        for line in stdout.lines().map(
            |x| x.unwrap_or("".to_string())
        ) {
            let mcserver_lock;
            if let Ok(lock) = Self::get_lock_nonblocking(&mcserver) {
                mcserver_lock = lock;
            } else {
                // the MCServer got corrupted and is now restarting
                return;
            }
            
            Self::save_output(&line, &mcserver_lock);

            if !agreed_to_eula {
                if let Err(_) = Self::agree_to_eula(&mcserver_lock) { // the error returned will always be a fatal one
                    Self::self_stop(&mcserver);
                    return;
                }
                agreed_to_eula = true;
            }

            drop(mcserver_lock);
            
            if !started {
                match Self::check_started(&line, start_time, &mcserver, log_messages) {
                    Ok(result) => started = result,
                    Err(erro) => {
                        match erro {
                            MCServerError::CriticalError => {
                                return;
                            }
                            // this will handle:
                            //      MCServerTypeError::InvalidFile
                            //      MCServerTypeError::FileNotFound
                            //      MCServerTypeError::ServerTypeNotFound
                            _ => {
                                unimplemented!("Something went wrong with the server_types.json file => The console needs to be implemented before deciding what to do here")
                            }
                        }
                    }
                }
            }

            if let Err(_) = Self::check_player_activity(&line, &mcserver) {
                Self::self_restart(&mcserver);
                return;
            }
        }
    }

    /// Save a given line to a log file saved under ' logs/{MCServer.name}.txt '.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                         | Description                                                             |
    /// |-----------------------------------|-------------------------------------------------------------------------|
    /// | `line: &str`                      | The string to save in the log file.                                     |
    /// | `mcserver: &Arc<Mutex<MCServer>>` | A reference to the MCServer struct which started this Minecraft server. |
    fn save_output(line: &str, mcserver_lock: &MutexGuard<MCServer<C>>) {
        match File::options().append(true).create_new(true).open(format!("logs/{}.txt", mcserver_lock.name)) {
            Ok(mut log_file) => {
                loop {
                    if let Ok(_) = log_file.write_all(format!("{line}\n").as_bytes()) {
                        break;
                    }
                }
            }
            Err(erro) => {
                match erro.kind() {
                    ErrorKind::NotFound => {
                        fs::create_dir("logs").unwrap(); // no error is expected, so we unwrap here

                        let mut log_file = File::options().append(true).create_new(true).open(format!("logs/{}.txt", mcserver_lock.name)).unwrap(); // no error is expected, so we unwrap here
                        loop {
                            if let Ok(_) = log_file.write_all(format!("{line}\n").as_bytes()) {
                                break;
                            }
                        }
                    }
                   ErrorKind::AlreadyExists => {                        
                        let mut log_file = File::options().append(true).open(format!("logs/{}.txt", mcserver_lock.name)).unwrap(); // no error is expected, so we unwrap here
                        loop {
                            if let Ok(_) = log_file.write_all(format!("{line}\n").as_bytes()) {
                                break;
                            }
                        }
                    }
                    _ => {
                        panic!("An unhandled error occurred while writing a line to the log file of {}.", mcserver_lock.name)
                    }
                }
            }
        }
    }
    /// Get the stdout pipe of the Minecraft server. This function will not handle errors.
    fn get_stdout_pipe(mcserver_lock: &mut MutexGuard<MCServer<C>>) -> Result<ChildStdout, MCServerError> {
        if let Some(child ) = mcserver_lock.minecraft_server.as_mut() {
            if let Some(childstdout) = child.stdout.take() {
                Ok(childstdout)
            } else {
                log!("erro", mcserver_lock.name, "The stdout pipe of this Minecraft server process does not exist. This MCServer will be restarted.");
                return Err(MCServerError::CriticalError);
            }
        } else {
            log!("erro", mcserver_lock.name, "The Minecraft server process could not be found.");
            return Err(MCServerError::CriticalError); // there is no need to properly stop the Minecraft server because none is running
        }
    }
    /// Check if the Minecraft server has started.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                            | Description                                               |
    /// |--------------------------------------|-----------------------------------------------------------|
    /// | `line: &str`                         | The line of text to analyze.                              |
    /// | `start_time: Instant`                | The time the Minecraft server was started.                |
    /// | `mcserver: &Arc<Mutex<MCServer<C>>>` | The MCServer to check for.                                |
    /// | `silent: bool`                       | Controls whether or not a started-message should be sent. |
    fn check_started(line: &str, start_time: Instant, mcserver: &Arc<Mutex<MCServer<C>>>, log_messages: bool) -> Result<bool, MCServerError> {
        let mut mcserver_lock = Self::get_lock_nonblocking(&mcserver)?;
        let mcserver_type = Self::get_mcserver_type(&mcserver_lock, &mcserver)?;
        
        for item in mcserver_type.get_started()? {
            if !line.contains(&item) {
                return Ok(false);
            }
        }

        if log_messages {
            log!("info", mcserver_lock.name, "Started in {:.3} secs!", start_time.elapsed().as_secs_f64());
        }
        mcserver_lock.status = MCServerStatus::Started;
        return Ok(true);
    }
    /// Check for player activity ( connecting/disconnecting ) and save the name of the player who joined or delete the one who left.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                         | Description                                                             |
    /// |-----------------------------------|-------------------------------------------------------------------------|
    /// | `line: &str`                      | The line of text to analyze.                                            |
    /// | `mcserver: &Arc<Mutex<MCServer>>` | A reference to the MCServer struct which started this Minecraft server. |
    fn check_player_activity(line: &str, mcserver: &Arc<Mutex<MCServer<C>>>) -> Result<(), MCServerError> {
        let mut mcserver_lock = Self::get_lock_nonblocking(&mcserver)?;
        let mcserver_type = Self::get_mcserver_type(&mcserver_lock, &mcserver)?;
        
        // check if anyone joined / left
        let mut player_joined = true;
        for item in mcserver_type.get_player_joined()? {
            if !line.contains(&item) {
                player_joined = false;
                break;
            }
        }
        let mut player_left = true;
        if !player_joined {
            for item in mcserver_type.get_player_left()? {
                if !line.contains(&item) {
                    player_left = false;
                    break;
                }
            }
        }
        
        // save the detected state to this MCServer
        if player_joined {
            mcserver_lock.players.push(mcserver_type.get_player_name_joined(&line)?);
        } else if player_left {
            let player_name = mcserver_type.get_player_name_left(&line)?;
            if let Ok(index) = mcserver_lock.players.binary_search(&player_name) {
                mcserver_lock.players.remove(index);
            } else {
                log!("erro", mcserver_lock.name, "The player {player_name} left without ever joining this server.");
                return Err(MCServerError::CriticalError);
            }
        }
        Ok(())
    }
    /// Automatically agree to the EULA if activated in the config. \
    /// If this setting is deactivated by the user, this function will send a message informing the user of the situation and then return an error to shut down the
    /// MCServer calling this function.
    fn agree_to_eula(mcserver_lock: &MutexGuard<MCServer<C>>) -> Result<(), MCServerError>{
        // check if the EULA has been accepted
        if Path::new(&(mcserver_lock.path.clone() + "/eula.txt")).exists() {
            let mut eula_txt = "".to_string();
            if let Err(_) = File::options().read(true).open(mcserver_lock.path.clone() + "/eula.txt").unwrap().read_to_string(&mut eula_txt) { }

            if eula_txt.contains("eula=true") {
                return Ok(());
            }
        }
        log!("warn", mcserver_lock.name, "The EULA has to be accepted to use this MCServer.");

        // agree to the EULA if configured
        if *mcserver_lock.config.agree_to_eula() {
            match File::create(mcserver_lock.path.clone() + "/eula.txt") {
                Ok(mut eula_file) => {
                    let failcounter = 0;
                    loop {
                        if let Err(_) = eula_file.write(b"eula=true") {
                            if failcounter == *mcserver_lock.config.max_tries() {
                                log!("erro", mcserver_lock.name, "The maximum number of write attempts to the ' eula.txt ' file have been reached. The MCServer will no longer try to accept the EULA.");
                                return Err(MCServerError::FatalError);
                            } else {
                                log!("erro", mcserver_lock.name, "This was attempt number {} out of {}", failcounter, mcserver_lock.config.max_tries());
                            }
                            thread::sleep(*mcserver_lock.config.refresh_rate());
                        } else {
                            break;
                        }
                    }
                }
                Err(erro) => {
                    log!("erro", mcserver_lock.name, "Failed to open the eula.txt file of this Minecraft server. Error: {erro}");
                    return Err(MCServerError::FatalError);
                }
            }
            
            log!("info", mcserver_lock.name, "#########################################################################################################################");
            log!("info", mcserver_lock.name, "# The following line is copied from the Minecraft Servers eula.txt file.                                                #");
            log!("info", mcserver_lock.name, "# `By changing the setting below to TRUE you are indicating your agreement to our EULA (https://aka.ms/MinecraftEULA).` #");
            log!("info", mcserver_lock.name, "# The EULA has been automatically accepted.                                                                             #");
            log!("info", mcserver_lock.name, "# To deactivate this behavior, change the ' agree_to_eula ' variable in the given config to false.                      #");
            log!("info", mcserver_lock.name, "#########################################################################################################################");
            
            return Ok(());
        } else {
            log!("warn", mcserver_lock.name, "#########################################################################################################################");
            log!("warn", mcserver_lock.name, "# The following line is copied from the Minecraft Servers eula.txt file.                                                #");
            log!("warn", mcserver_lock.name, "# `By changing the setting below to TRUE you are indicating your agreement to our EULA (https://aka.ms/MinecraftEULA).` #");
            log!("warn", mcserver_lock.name, "# The EULA has not yet been accepted. Please accept it to continue using this server.                                   #");
            log!("warn", mcserver_lock.name, "# To automatically accept all EULAs in the future, change the ' agree_to_eula ' variable in the given config to true.   #");
            log!("warn", mcserver_lock.name, "#                                                                                                                       #");
            log!("warn", mcserver_lock.name, "# This MCServer will now shut down.                                                                                     #");
            log!("warn", mcserver_lock.name, "#########################################################################################################################");
        
            return Err(MCServerError::FatalError)
        }
    }
}