//! This module provides the [`MCServer struct`](MCServer) which represents an API for one Minecraft server, which got assigned with the initiation of this struct.


use std::fs::{File, self};
use std::io::{BufReader, BufRead, Write, Read, ErrorKind};
use std::path::Path;
use std::process::{Command, Child, Stdio, ChildStdout};
use std::sync::{Mutex, Arc, MutexGuard};
use std::thread;
use std::time::Instant;

use crate::log;
use crate::config::Config;
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
pub struct MCServer<C: Config> {
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
impl<C: Config> MCServer<C> {
    /// The default state of the MCServer struct.
    fn default_state(name: String, arg: Vec<String>, path: String, mcserver_type: Arc<Mutex<MCServerType>>, config: Arc<C>) -> Self {
        Self {
            name,
            arg,
            path,
            mcserver_type,
            config,

            minecraft_server: None,
            main_thread: None,
            
            alive: false,
            status: MCServerStatus::Stopped,
            players: vec![],
        }
    }
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
        Arc::new(Mutex::new(Self::default_state(
            name.to_owned(),
            arg.split(" ").map(String::from).collect(),
            format!("servers/{}", name),
            Arc::new(Mutex::new(mcserver_type)),
            config.clone()
        )))
    }
    /// Reset the MCServer to its default state.The MCServer provided has to be locked behind a mutex. \
    /// If you want to unlock the MCServer yourself use the [`reset_unlocked function`](MCServer::reset_unlocked).
    fn reset(mcserver: &Arc<Mutex<MCServer<C>>>) {
        match mcserver.lock() {
            Ok(mut mcserver) => {
                Self::reset_unlocked(&mut mcserver);
            }
            Err(mcserver) => {
                Self::reset_unlocked(&mut mcserver.into_inner());
            }
        }
    }
    /// Reset the MCServer to its default state. The MCServer provided has to be unlocked. \
    /// If you do not want to unlock the MCServer yourself use the [`reset function`](MCServer::reset).
    fn reset_unlocked(mcserver: &mut MutexGuard<MCServer<C>>) {
        **mcserver = Self::default_state(
            mcserver.name.clone(),
            mcserver.arg.clone(),
            mcserver.path.clone(),
            mcserver.mcserver_type.clone(),
            mcserver.config.clone()
        )
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

    /// Get the given mcserver's lock. \
    /// This function will block the thread calling until the lock is claimed. If an error occurs, this function will return None and do no error recovering. \
    /// To guarantee getting the lock use the [`get_lock`](Self::get_lock) function.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                         | Description                                              |
    /// |-----------------------------------|----------------------------------------------------------|
    /// | `mcserver: &Arc<Mutex<MCServer>>` | The MCServer to obtain the lock from.                    |
    /// | `error_message: bool`             | Controls whether or not an error message should be send. |
    fn get_lock_pure(mcserver: &Arc<Mutex<MCServer<C>>>, error_message: bool) -> Option<MutexGuard<MCServer<C>>> {
        match mcserver.lock() {
            Ok(mcserver_lock) => {
                return Some(mcserver_lock);
            }
            Err(erro) => { 
                if error_message { let name = erro.into_inner().name.clone(); log!("erro", name, "This MCServer got corrupted! It will be restarted."); }
                return None;
            }
        }
    }
    /// Get the given mcserver's lock. \
    /// This function will block the thread calling until the lock is claimed. If an error occurs, this function will restart the mcserver. \
    /// Use the [`get_lock_nonblocking`](Self::get_lock_nonblocking) function to run this restart in a different thread.
    fn get_lock(mcserver: &Arc<Mutex<MCServer<C>>>) -> MutexGuard<MCServer<C>> {
        if let Some(mcserver_lock) = Self::get_lock_pure(mcserver, true) {
            return mcserver_lock;
        }
        if let Err(_) = Self::restart(mcserver) {
            Self::reset(&mcserver);
        }
        
        return Self::get_lock(mcserver);
    }
    /// Get the given mcserver's lock. \
    /// This function will block the thread calling until the lock is claimed. \
    /// If an error occurs, this function will restart the mcserver in a different thread and return an error.\
    /// To guarantee getting the lock use the [`get_lock`](Self::get_lock) function.
    fn get_lock_nonblocking(mcserver: &Arc<Mutex<MCServer<C>>>) -> Result<MutexGuard<MCServer<C>>, MCServerError> {
        let mcs = mcserver.clone();
        if let Some(mcserver_lock) = Self::get_lock_pure(mcserver, true) {
            return Ok(mcserver_lock);
        }
        thread::spawn(move || {
            if let Err(_) = Self::restart(&mcs) {
                Self::reset(&mcs);
            }
        });

        return Err(MCServerError::CriticalError);
    }

    /// Start the [`MCServer`].
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                           | Description                                                             |
    /// |-------------------------------------|-------------------------------------------------------------------------|
    /// | `mcserver: &Arc<Mutex<MCServer>>`   | A reference to the MCServer struct which started this Minecraft server. |
    /// | `restart: bool`                     | Controls whether or not the start is used in a restart.                 |
    pub fn start(mcserver: &Arc<Mutex<MCServer<C>>>, restart: bool) -> Result<(), MCServerError>{
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
                log!("erro", &name, "An error occurred while starting the Minecraft Server {name}. Error: {err}");
                Self::reset_unlocked(&mut mcserver_lock);
                return Err(MCServerError::FailedCommandSpawn(name.clone(), err))
            }
        }
        
        mcserver_lock.alive = true;

        let silent = restart;
        mcserver_lock.main_thread = Some(thread::spawn(move ||
            Self::main(mcserver_clone, silent)
        ));

        Ok(())
    }
    /// Stop the [`MCServer`].
    /// 
    /// ## Parameters
    /// 
    /// | Parameter                         | Description                                                             |
    /// |-----------------------------------|-------------------------------------------------------------------------|
    /// | `mcserver: &Arc<Mutex<MCServer>>` | A reference to the MCServer struct which started this Minecraft server. |
    /// | `restart: bool`                     | Controls whether or not the start is used in a restart.                 |
    pub fn stop(mcserver: &Arc<Mutex<MCServer<C>>>, restart: bool) -> Result<(), MCServerError> {
        let mut mcserver_lock;
        if let Some(lock) = Self::get_lock_pure(mcserver, false) {
            mcserver_lock = lock;
        } else {
            if !restart { log!("erro", "MCServer", "A MCServer got corrupted."); }
            MCServer::reset(&mcserver);
            return Err(MCServerError::FatalError);
        }


        let name = mcserver_lock.name.clone();
        let stop_time = Instant::now();

        // check if the MCServer has started
        if restart {
        } else if mcserver_lock.status != MCServerStatus::Started {
            return Err(MCServerError::NotStarted);
        } else if mcserver_lock.status == MCServerStatus::Stopped {
            // do nothing since this server is already stopped
            return Ok(());
        }
        

        if !restart { log!("info", &name, "Stopping..."); }

        mcserver_lock.status = MCServerStatus::Stopping;

        if let Some(mut minecraft_server ) = mcserver_lock.minecraft_server.take() {
            // send the stop command to the Minecraft server
            if let Some(stdin) = minecraft_server.stdin.as_mut() {
                if let Err(err) = stdin.write_all(format!("stop\n").as_bytes()) {
                    if !restart { log!("erro", &name, "An error occurred while writing the input `stop` to the Minecraft server. The process will be kill forcefully. Error: {err}"); }
                    if let Err(_) = minecraft_server.kill() {}
                }
            } else {
                if !restart { log!("erro", &name, "The stdin pipe of this Minecraft server process does not exist. The process will be kill forcefully."); }
                if let Err(_) = minecraft_server.kill() {}
            }

            // wait for the Minecraft server to finish
            if let Err(err) = minecraft_server.wait() {
                if !restart { log!("erro", &name, "An error occurred while waiting on the Minecraft server to finish. Error: {err}"); }
                Self::reset_unlocked(&mut mcserver_lock);
                return Err(MCServerError::FatalError);
            }
        } else {
            if !restart { log!("erro", &name, "Could not get the Minecraft server. It was already taken."); }
            Self::reset_unlocked(&mut mcserver_lock);
            return Err(MCServerError::FatalError);
        }

        // give the shutdown command
        mcserver_lock.alive = false;
        
        // acquire the main thread
        let main_thread;
        if let Some(main) = mcserver_lock.main_thread.take() {
            main_thread = main;
        } else {
            if !restart { log!("erro", &name, "Could not take the main thread. It was already taken."); }
            Self::reset_unlocked(&mut mcserver_lock);
            return Err(MCServerError::FatalError);
        }

        drop(mcserver_lock);

        // wait for the main thread to finish
        if let Err(_) = main_thread.join() {
            if !restart { log!("erro", "MCServer", "Failed to join the main thread."); }
            MCServer::reset(&mcserver);
            return Err(MCServerError::FatalError);
        }

        // set the MCServers status to stopped
        if let Ok(mut mcserver_lock) = mcserver.lock() {
            mcserver_lock.status = MCServerStatus::Stopped;
        } else {
            if !restart { log!("erro", &name, "This MCServer got corrupted."); }
            MCServer::reset(&mcserver);
            return Err(MCServerError::FatalError);
        }

        if !restart { log!("info", &name, "Stopped in {:.3} secs!", stop_time.elapsed().as_secs_f64()); }

        Ok(())
    }
    /// Restart the [`MCServer`]. \
    /// This function will re-initiate the provided MCServer.
    pub fn restart(mcserver: &Arc<Mutex<MCServer<C>>>) -> Result<(), MCServerError> {
        let restart_time = Instant::now();
        
        // get all the necessary data from the old struct
        let name;
        let config;
        match mcserver.lock() {
            Ok(mut mcserver) => {
                // check if the MCServer has started
                if mcserver.status != MCServerStatus::Started {
                    return Err(MCServerError::NotStarted);
                }

                mcserver.status = MCServerStatus::Restarting;
                
                name = mcserver.name.clone();
                config = mcserver.config.clone();
            }
            Err(mcs) => {
                log!("erro", "MCServer", "Found that a MCServer was corrupted during restart. It will be reset.");
                Self::reset(&mcserver);

                let mcs = mcs.into_inner();
                name = mcs.name.clone();
                config = mcs.config.clone();
            }
            
        }

        log!("info", &name, "Restarting...");


        // ### STOPPING ###

        // Try to stop the mcserver until it succeeds, and reset it afterwards
        loop {
            match Self::stop(&mcserver, true) {
                Ok(_) => {
                    MCServer::reset(&mcserver);
                    break;
                }
                Err(err) => {
                    match err {
                        MCServerError::FatalError => {
                            break;
                        }
                        _ => {
                            // The only case in which this loop never ends is when the Minecraft server gets deadlocked
                            // and therefore never completes its startup process.
                            thread::sleep(*config.refresh_rate());
                        }
                    }
                }
            }
        }
        

        // ### STARTING ###


        // Try to start the mcserver until it succeeds or the fail limit is reached
        let failcounter = 0;
        loop {
            if let Err(_) = Self::start(&mcserver, true) {
                if failcounter == *config.max_tries() {
                    log!("erro", &name, "The maximum number of start attempts has been reached. The MCServer will no longer attempt to start.");
                    MCServer::reset(&mcserver);
                    return Err(MCServerError::FatalError);
                } else {
                    log!("erro", &name, "This was attempt number {} out of {}", failcounter, config.max_tries());
                }
                thread::sleep(*config.refresh_rate());
            } else {
                break;
            }
        }

        // wait for the Minecraft server to complete its startup
        loop {
            if let MCServerStatus::Started = Self::get_status(&mcserver)? {
                log!("info", &name, "Restarted in {:.3} secs!", restart_time.elapsed().as_secs_f64());
                return Ok(());
            } else {
                thread::sleep(*config.refresh_rate());
            }
        }
    }
    /// This method gets used to restart the [`MCServer`] without blocking the thread calling it. \
    /// If you want to block the thread calling this function use the [`restart function`](MCServer::restart).
    fn self_restart(mcserver: &Arc<Mutex<MCServer<C>>>) {
        let mcs = mcserver.clone();
        thread::spawn(move || 
            loop {
                if let Ok(_) = Self::restart(&mcs) {
                    break;
                }
            }
        );
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
                            if let MCServerError::NotStarted = erro {
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
                        if let MCServerError::NotStarted = erro {
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
    fn main(mcserver: Arc<Mutex<MCServer<C>>>, silent: bool) {
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

        if !silent {
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
                if let Err(_) = Self::agree_to_eula(&mcserver_lock) {
                    Self::self_restart(&mcserver);
                    return;
                }
                agreed_to_eula = true;
            }

            drop(mcserver_lock);
            
            if !started {
                match Self::check_started(&line, start_time, &mcserver, silent) {
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
    fn check_started(line: &str, start_time: Instant, mcserver: &Arc<Mutex<MCServer<C>>>, silent: bool) -> Result<bool, MCServerError> {
        let mut mcserver_lock = Self::get_lock_nonblocking(&mcserver)?;
        let mcserver_type = Self::get_mcserver_type(&mcserver_lock, &mcserver)?;
        
        for item in mcserver_type.get_started()? {
            if !line.contains(&item) {
                return Ok(false);
            }
        }

        if !silent {
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