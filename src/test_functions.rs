#![cfg(test)]


use std::io::ErrorKind;
use std::net::{SocketAddrV4, Ipv4Addr};
use std::time::Duration;
use std::{fs, io};
use std::path::Path;

use crate::config_trait::ConfigTrait;


// The following line is copied from the Minecraft servers EULA
// By changing the setting below to TRUE you are indicating your agreement to our EULA (https://aka.ms/MinecraftEULA).
const AGREE_TO_EULA: bool = false;
// Note: By changing the following constant, it can happen that the computer running one of these tests shuts down.
const AUTO_START: bool = false;
const MCSERVER_RESTART_TIME: bool = false;

fn get_duration(bool: bool) -> Duration {
    if bool {
        Duration::new(60, 0)
    } else {
        Duration::new(0, 0)
    }
}


pub struct Config {
    addr: SocketAddrV4,
    buffsize: u32,
    refresh_rate: Duration,
    max_tries: i32,
    agree_to_eula: bool,
    shutdown_time: Duration,
    mcserver_restart_time: Duration
}
impl ConfigTrait for Config {
    fn new() -> Self {
        Self {
            addr: SocketAddrV4::new(Ipv4Addr::LOCALHOST, 25564),
            buffsize: 100000000,
            refresh_rate: Duration::new(0, 100000000),
            max_tries: 3,
            agree_to_eula: AGREE_TO_EULA,
            shutdown_time: get_duration(AUTO_START),
            mcserver_restart_time: get_duration(MCSERVER_RESTART_TIME),
        }
    }
    fn addr(&self) -> &SocketAddrV4 {
        &self.addr
    }
    fn buffsize(&self) -> &u32 {
        &self.buffsize
    }
    fn refresh_rate(&self) -> &Duration {
        &self.refresh_rate
    }
    fn max_tries(&self) -> &i32 {
        &self.max_tries
    }
    fn agree_to_eula(&self) -> &bool {
        &self.agree_to_eula
    }
    fn shutdown_time(&self) -> &Duration {
        &self.shutdown_time
    }
    fn mcserver_restart_time(&self) -> &Duration {
        &self.mcserver_restart_time
    }
}


pub fn cleanup() {
    if let Err(_) = cleanup_dir("./servers/") {}
    if let Err(_) = cleanup_dir("./config/") {}
    if let Err(_) = cleanup_dir("./logs/") {}
}
pub fn cleanup_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let path = entry.path();

        if entry.file_type()?.is_dir() {
            cleanup_dir(&path)?;
            if let Err(erro) = fs::remove_dir(&path) {
                match erro.kind() {
                    ErrorKind::NotFound => {}
                    _ => {
                        return Err(erro);
                    }
                }
            }
        } else {
            fs::remove_file(path)?;
        }
    }
    fs::remove_dir(path)?;
    Ok(())
}