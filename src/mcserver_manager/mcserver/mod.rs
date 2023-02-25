//! This module provides the [`MCServer struct`](MCServer) which represents an API for one Minecraft server, which got assigned with the initiation of this struct.


use std::fs::{File, self};
use std::io::{Write, ErrorKind, Read};
use std::process::Stdio;
use std::sync::atomic::AtomicBool;
use std::thread::JoinHandle;
use std::{str, thread};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::Ordering::Relaxed;
use std::time::Instant;

use async_recursion::async_recursion;
use async_trait::async_trait;
use tokio::io::{AsyncWriteExt, BufReader, AsyncBufReadExt};
use tokio::sync::oneshot::{self, channel, Sender};
use tokio::sync::Mutex;
use tokio::time::{self, sleep};
use tokio::process::{Command, Child, ChildStdout};
use tokio::runtime::Runtime;

use crate::concurrent_class::ConcurrentClass;
use crate::concurrent_class::qol_functions::{check_allowed_start, check_allowed_stop};
use crate::concurrent_class::status::Status;
use crate::config::Config;
use crate::log;
use crate::mcmanage_error::MCManageError;
use mcserver_type::MCServerType;


mod tests;
pub mod mcserver_type;


/// This struct represents an API for one Minecraft server, which got assigned with the initiation of this struct. \
/// 
/// 
/// # Features
/// 
/// - The log of the Minecraft server running gets saved to ' logs/{MCServer.name}.txt '.
/// - Lines of text can be sent to the Minecraft server.
/// - The names of the players currently on the Minecraft server get saved.
/// - The [`status`](Status) of the Minecraft server gets saved. ( Starting, Stopping, ... )
/// - Automatically agrees to the EULA if activated in the [`config`](Config).
/// 
/// 
/// # Methods
/// 
/// | Method                                    | Description                                                              |
/// |-------------------------------------------|--------------------------------------------------------------------------|
/// | [`new(...) -> Arc<Self>`](MCServer::new)  | Create a new [`MCServer`] instance.                                      |
/// | [`players(...)`](MCServer::players)       | Return a list of every player who is currently on this Minecraft server. |
/// | [`send_input(...)`](MCServer::send_input) | Send a given string to the Minecraft server as an input.                 |
/// 
/// ... and other functions inherited by the [`ConcurrentClass trait`](ConcurrentClass).
pub struct MCServer {
    name: String,
    arg: Vec<String>,
    path: String,
    mcserver_type: MCServerType,
    config: Arc<Config>,

    minecraft_server: Mutex<Option<Child>>,
    main_thread: Mutex<Option<JoinHandle<()>>>, // std JoinHandle needs to be used here because else the main thread will not work properly
    
    alive: AtomicBool,
    status: Mutex<Status>,
    players: Mutex<Vec<String>>
}
#[async_trait]
impl ConcurrentClass for MCServer {
    fn name(self: &Arc<Self>) -> String {
        self.name.clone()
    }
    fn config(self: &Arc<Self>) -> Arc<Config> {
        self.config.clone()
    }
    async fn status(self: &Arc<Self>) -> Status {
        *self.status.lock().await
    }
    async fn set_status(self: &Arc<Self>, new_status: Status) {
        *self.status.lock().await = new_status
    }
    async fn reset(self: &Arc<Self>) {
        *self.minecraft_server.lock().await = None;
        *self.main_thread.lock().await = None;
        self.alive.store(false, Relaxed);
        *self.status.lock().await = Status::Stopped;
        *self.players.lock().await = vec![];
    }
    async fn impl_start(self: Arc<Self>, restart: bool) -> Result<(), MCManageError> {
        check_allowed_start(&self, restart).await?;
        
        let start_time = Instant::now();
        if !restart { log!("info", self.name, "Starting..."); }

        match Command::new("java")
            .current_dir(&self.path)
            .args(&self.arg)
            .stderr(Stdio::inherit())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Ok(minecraft_server) => {
                *self.minecraft_server.lock().await = Some(minecraft_server);
            }
            Err(err) => {
                log!("erro", self.name, "An error occurred while starting the Minecraft Server {}. Error: {err}", self.name);
                self.reset().await;
                return Err(MCManageError::FatalError)
            }
        }
        
        self.alive.store(true, Relaxed);
        let (tx, rx) = channel();

        let mcserver = self.clone();
        *self.main_thread.lock().await = Some(thread::spawn(move || {
            let runtime = Runtime::new().unwrap();
            if let Err(_) = runtime.block_on(mcserver.clone().main(Some(tx))) {}
        }));

        self.recv_start_result(rx).await?;

        if !restart { log!("info", self.name, "Started in {:.3} secs!", start_time.elapsed().as_secs_f64()); }

        Ok(())
    }
    async fn impl_stop(self: Arc<Self>, restart: bool, forced: bool) -> Result<(), MCManageError> {
        check_allowed_stop(&self, restart, forced).await?;
        
        let stop_time = Instant::now();
        if !restart { log!("info", self.name, "Stopping..."); }

        if let Some(mut minecraft_server ) = self.minecraft_server.lock().await.take() {
            // send the stop command to the Minecraft server
            if let Some(stdin) = minecraft_server.stdin.as_mut() {
                if let Err(erro) = stdin.write_all(format!("stop\n").as_bytes()).await {
                    if !restart { log!("warn", self.name, "An error occurred while writing the input `stop` to the Minecraft server. The process will be kill forcefully. Error: {erro}"); }
                    if let Err(_) = minecraft_server.kill().await {}
                }
                self.save_output(">> stop").await;
            } else {
                if !restart { log!("warn", self.name, "The stdin pipe of this Minecraft server process does not exist. The process will be kill forcefully."); }
                if let Err(_) = minecraft_server.kill().await {}
            }

            // wait for the Minecraft server to finish
            if let Err(erro) = minecraft_server.wait().await {
                log!("erro", self.name, "An error occurred while waiting on the Minecraft server to finish. Error: {erro}");
                self.reset().await;
                return Err(MCManageError::FatalError);
            }
        }

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

        *self.status.lock().await = Status::Stopped;

        if !restart { log!("info", self.name, "Stopped in {:.3} secs!", stop_time.elapsed().as_secs_f64()); }

        Ok(())
    }
    async fn main(self: Arc<Self>, mut bootup_result: Option<Sender<()>>) -> Result<(), MCManageError> {
        let mut agreed_to_eula = false;
        let stdout = BufReader::new(self.get_stdout_pipe().await?);

        let mut lines = stdout.lines();
        while self.alive.load(Relaxed) {
            let line;
            match lines.next_line().await {
                Ok(content) => {
                    if let Some(content) = content {
                        line = content;
                    } else {
                        // It will only be None returned if the Child process got killed
                        return Ok(())
                    }
                }
                Err(erro) => {
                    unimplemented!("An error occurred while reading the output of {}. Error: {erro}", self.name)
                }
            }

            self.save_output(&line).await;
            
            if !agreed_to_eula {
                self.agree_to_eula().await?;
                agreed_to_eula = true;
            }
            
            if let Some(bootup_result_inner) = bootup_result {
                match self.check_started(&line, bootup_result_inner).await {
                    Ok(result) => bootup_result = result,
                    Err(erro) => {
                        match erro {
                            MCManageError::CriticalError => {
                                return Err(MCManageError::CriticalError);
                            }
                            // this will handle:
                            //      MCServerTypeError::InvalidFile
                            //      MCServerTypeError::FileNotFound
                            //      MCServerTypeError::NotFound
                            _ => {
                                // TODO: Something went wrong with the server_types.json file => Handle this error in the MCServer
                                unimplemented!("Something went wrong with the server_types.json file => The console needs to be implemented before deciding what to do here")
                            }
                        }
                    }
                }
            }

            self.check_player_activity(&line).await?;
        }

        Ok(())
    }
}     
impl MCServer {
    /// Create a new [`MCServer`] instance.
    pub fn new(name: &str, arg: &str, mcserver_type: MCServerType, config: &Arc<Config>) -> Arc<Self> {
        Arc::new(Self {
            name: name.to_owned(),
            arg: arg.split(" ").map(String::from).collect(),
            path: format!("servers/{}", name),
            mcserver_type,
            config: config.clone(),

            minecraft_server: None.into(),
            main_thread: None.into(),
            
            alive: AtomicBool::new(false),
            status: Status::Stopped.into(),
            players: vec![].into(),
        })
    }

    /// Return a list of every player who is currently on this Minecraft server.
    pub async fn players(self: &Arc<Self>) -> Vec<String> {
        self.players.lock().await.clone()
    }

    /// Send a given string to the Minecraft server as an input. \
    /// It is guaranteed that the string given will be sent to the MCServer, but this can cause the blocking of the thread calling this function due to the MCServer restarting.
    #[async_recursion]
    pub async fn send_input(self: &Arc<Self>, input: &str) {
        if let Some(child) = self.minecraft_server.lock().await.as_mut() {
            if let Some(stdin) = child.stdin.as_mut() {
                if let Err(erro) = stdin.write_all(format!("{input}\n").as_bytes()).await {
                    log!("erro", self.name, "An error occurred while writing the input `{input}` to the Minecraft server. This MCServer will be restarted. Error: {erro}");
                    loop {
                        if let Err(erro) = self.clone().impl_restart().await {
                            if let MCManageError::NotReady = erro {
                                sleep(*self.config.refresh_rate()).await;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    self.send_input(input).await;
                }
                self.save_output(&format!(">> {input}")).await;
            } else {
                log!("erro", self.name, "The stdin pipe of this Minecraft server process does not exist. This MCServer will be restarted.");
                loop {
                    if let Err(erro) = self.clone().impl_restart().await {
                        if let MCManageError::NotReady = erro {
                            sleep(*self.config.refresh_rate()).await;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                self.send_input(input).await;
            }
        } else {
            log!("erro", self.name, "The Minecraft server process could not be found. Please start the Minecraft server before sending input to it.");
            return; // there is no need to properly stop the Minecraft server because none is running
        }
    }

    /// Save a given line to a log file saved under ' logs/{MCServer.name}.txt '.
    async fn save_output(self: &Arc<Self>, line: &str) {
        match File::options().append(true).create_new(true).open(format!("logs/{}.txt", self.name)) {
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

                        let mut log_file = File::options().append(true).create_new(true).open(format!("logs/{}.txt", self.name)).unwrap(); // no error is expected, so we unwrap here
                        loop {
                            if let Ok(_) = log_file.write_all(format!("{line}\n").as_bytes()) {
                                break;
                            }
                        }
                    }
                   ErrorKind::AlreadyExists => {                        
                        let mut log_file = File::options().append(true).open(format!("logs/{}.txt", self.name)).unwrap(); // no error is expected, so we unwrap here
                        loop {
                            if let Ok(_) = log_file.write_all(format!("{line}\n").as_bytes()) {
                                break;
                            }
                        }
                    }
                    _ => {
                        panic!("An unhandled error occurred while writing a line to the log file of {}.", self.name)
                    }
                }
            }
        }
    }
    /// Get the stdout pipe of the Minecraft server. This function will not handle errors.
    async fn get_stdout_pipe(self: &Arc<Self>) -> Result<ChildStdout, MCManageError> {
        if let Some(child ) = self.minecraft_server.lock().await.as_mut() {
            if let Some(childstdout) = child.stdout.take() {
                return Ok(childstdout);
            } else {
                log!("erro", self.name, "The stdout pipe of this Minecraft server process does not exist. This MCServer will be restarted.");
            }
        } else {
            log!("erro", self.name, "The Minecraft server process could not be found.");
        }
        self.restart();
        return Err(MCManageError::CriticalError);
    }
    /// Check if the Minecraft server has started.
    async fn check_started(self: &Arc<Self>, line: &str, bootup_result: oneshot::Sender<()>) -> Result<Option<oneshot::Sender<()>>, MCManageError> {
        for item in self.mcserver_type.get_started().await? {
            if !line.contains(&item) {
                return Ok(Some(bootup_result));
            }
        }
        self.send_start_result(bootup_result).await?;
        *self.status.lock().await = Status::Started;
        return Ok(None);
    }
    /// Check for player activity ( connecting/disconnecting ) and save the name of the player who joined or delete the one who left.
    async fn check_player_activity(self: &Arc<Self>, line: &str) -> Result<(), MCManageError> {
        // check if anyone joined / left
        let mut player_joined = true;
        for item in self.mcserver_type.get_player_joined().await? {
            if !line.contains(&item) {
                player_joined = false;
                break;
            }
        }
        let mut player_left = true;
        if !player_joined {
            for item in self.mcserver_type.get_player_left().await? {
                if !line.contains(&item) {
                    player_left = false;
                    break;
                }
            }
        }
        
        // save the detected state to this MCServer
        let mut players = self.players.lock().await;
        if player_joined {
            players.push(self.mcserver_type.get_player_name_joined(&line).await?);
        } else if player_left {
            let player_name = self.mcserver_type.get_player_name_left(&line).await?;
            if let Ok(index) = players.binary_search(&player_name) {
                players.remove(index);
            } else {
                log!("erro", self.name, "The player {player_name} left without ever joining this server.");

                self.restart();
                return Err(MCManageError::CriticalError);
            }
        }
        Ok(())
    }
    /// Automatically agree to the EULA if activated in the config. \
    /// If this setting is deactivated by the user, this function will send a message informing the user of the situation and then return an error and shut down the
    /// MCServer calling this function.
    async fn agree_to_eula(self: &Arc<Self>) -> Result<(), MCManageError> {
        // check if the EULA has been accepted
        if Path::new(&(self.path.clone() + "/eula.txt")).exists() {
            let mut eula_txt = "".to_string();
            if let Err(_) = File::options().read(true).open(self.path.clone() + "/eula.txt").unwrap().read_to_string(&mut eula_txt) { }

            if eula_txt.contains("eula=true") {
                return Ok(());
            }
        }
        log!("warn", self.name, "The EULA has to be accepted to use this MCServer.");

        // agree to the EULA if configured
        if *self.config.agree_to_eula() {
            match File::create(self.path.clone() + "/eula.txt") {
                Ok(mut eula_file) => {
                    let failcounter = 0;
                    loop {
                        if let Err(_) = eula_file.write(b"eula=true") {
                            if failcounter == *self.config.max_tries() {
                                log!("erro", self.name, "The maximum number of write attempts to the ' eula.txt ' file have been reached. The MCServer will no longer try to accept the EULA.");
                                self.stop();
                                return Err(MCManageError::FatalError);
                            } else {
                                log!("erro", self.name, "This was attempt number {} out of {}", failcounter, self.config.max_tries());
                            }
                            time::sleep(*self.config.refresh_rate()).await;
                        } else {
                            break;
                        }
                    }
                }
                Err(erro) => {
                    log!("erro", self.name, "Failed to open the eula.txt file of this Minecraft server. Error: {erro}");
                    self.stop();
                    return Err(MCManageError::FatalError);
                }
            }
            
            log!("info", self.name, "#########################################################################################################################");
            log!("info", self.name, "# The following line is copied from the Minecraft Servers eula.txt file.                                                #");
            log!("info", self.name, "# `By changing the setting below to TRUE you are indicating your agreement to our EULA (https://aka.ms/MinecraftEULA).` #");
            log!("info", self.name, "# The EULA has been automatically accepted.                                                                             #");
            log!("info", self.name, "# To deactivate this behavior, change the ' agree_to_eula ' variable in the given config to false.                      #");
            log!("info", self.name, "#########################################################################################################################");
            
            return Ok(());
        } else {
            log!("warn", self.name, "#########################################################################################################################");
            log!("warn", self.name, "# The following line is copied from the Minecraft Servers eula.txt file.                                                #");
            log!("warn", self.name, "# `By changing the setting below to TRUE you are indicating your agreement to our EULA (https://aka.ms/MinecraftEULA).` #");
            log!("warn", self.name, "# The EULA has not yet been accepted. Please accept it to continue using this server.                                   #");
            log!("warn", self.name, "# To automatically accept all EULAs in the future, change the ' agree_to_eula ' variable in the given config to true.   #");
            log!("warn", self.name, "#                                                                                                                       #");
            log!("warn", self.name, "# This MCServer will now shut down.                                                                                     #");
            log!("warn", self.name, "#########################################################################################################################");
            
            self.stop();
            return Err(MCManageError::FatalError)
        }
    }
}