#![allow(non_snake_case)]
#![cfg(test)]


use std::fs;
use std::fs::File;
use std::io;
use std::net::{SocketAddrV4, Ipv4Addr};
use std::time::Duration;
use reqwest;

use super::*;
use crate::test_functions::*;

// The following line is copied from the Minecraft servers EULA
// By changing the setting below to TRUE you are indicating your agreement to our EULA (https://aka.ms/MinecraftEULA).
const AGREE_TO_EULA: bool = false;


struct MyConfig {
    addr: SocketAddrV4,
    buffsize: u32,
    refresh_rate: Duration,
    max_tries: i32,
    agree_to_eula: bool
}
impl Config for MyConfig {
    fn new() -> Self {
        Self {
            addr: SocketAddrV4::new(Ipv4Addr::LOCALHOST, 25564),
            buffsize: 100000000,
            refresh_rate: Duration::new(0, 100000000),
            max_tries: 3,
            agree_to_eula: AGREE_TO_EULA
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
}

fn new_mcserver<C: Config>() -> Arc<Mutex<MCServer<C>>> {
    cleanup();
    download_minecraft_server();

    MCServer::new(
        "myMinecraftServer",
        "-jar purpur-1.19.3-1876.jar nogui",
        MCServerType::new("purpur"),
        &Arc::new(C::new())
    )
}
fn new_mcserver_no_download<C: Config>() -> Arc<Mutex<MCServer<C>>> {
    cleanup();
    
    MCServer::new(
        "myMinecraftServer",
        "-jar purpur-1.19.3-1876.jar nogui",
        MCServerType::new("purpur"),
        &Arc::new(C::new())
    )
}
fn download_minecraft_server() {
    let mut resp = reqwest::blocking::get("https://api.purpurmc.org/v2/purpur/1.19.3/1876/download").expect("An error occurred while downloading the Minecraft server");
    fs::create_dir_all("servers/myMinecraftServer").expect("An error occurred while creating the servers dir");
    let mut out = File::create("servers/myMinecraftServer/purpur-1.19.3-1876.jar").expect("failed to create file `purpur-1.19.3-1876.jar`");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
}

// getter / setter functions
#[test]
fn MCServer__get_mcserver_type() {
    let mcserver = new_mcserver_no_download::<MyConfig>();

    let _mcserver_type = MCServer::get_mcserver_type(&MCServer::get_lock_pure(&mcserver, true).unwrap(), &mcserver).unwrap();
    assert!(true);
    cleanup();
    
    // This should work

    // if let MCServer = mcserver_type {
    // } else {
    //     assert!(false, "Expected mcserver_type to be MCServerType::Purpur.")
    // }
}
#[test]
fn MCServer__get_status() {
    let mcserver = new_mcserver_no_download::<MyConfig>();

    let status = MCServer::get_status(&mcserver).unwrap();
    
    if let MCServerStatus::Stopped = status {
    } else {
        assert!(false, "Expected MCServerStatus to be MCServerStatus::Stopped.")
    }

    cleanup();
}
#[test]
fn MCServer__get_players() {
    let mcserver = new_mcserver_no_download::<MyConfig>();

    let players = MCServer::get_players(&mcserver).unwrap();
    let expected_result: Vec<String> = vec![];
    assert_eq!(players, expected_result);

    cleanup();
}

#[test]
fn MCServer__reset() {
    let mcserver = new_mcserver_no_download::<MyConfig>();
    let mut mcserver_lock = mcserver.lock().unwrap();

    mcserver_lock.alive = true;
    mcserver_lock.status = MCServerStatus::Started;
    mcserver_lock.players = vec!["hello".to_owned()];

    drop(mcserver_lock);

    MCServer::reset(&mcserver);

    let mcserver_lock = mcserver.lock().unwrap();

    assert_eq!(mcserver_lock.alive, false, "Expected alive field to be false.");
    if let MCServerStatus::Stopped = mcserver_lock.status {
    } else {
        assert!(false, "Expected status field to be MCServerStatus::Stopped.");
    };
    assert_eq!(mcserver_lock.players.len(), 0, "Expected players field to be vec![].");
    cleanup();
}
#[test]
fn MCServer__reset_unlocked() {
    let mcserver = new_mcserver_no_download::<MyConfig>();
    let mut mcserver_lock = mcserver.lock().unwrap();

    mcserver_lock.alive = true;
    mcserver_lock.status = MCServerStatus::Started;
    mcserver_lock.players = vec!["hello".to_owned()];

    MCServer::reset_unlocked(&mut mcserver_lock);

    assert_eq!(mcserver_lock.alive, false, "Expected alive field to be false.");
    if let MCServerStatus::Stopped = mcserver_lock.status {
    } else {
        assert!(false, "Expected status field to be MCServerStatus::Stopped.");
    };
    assert_eq!(mcserver_lock.players.len(), 0, "Expected players field to be vec![].");
    cleanup();
}

#[test]
fn MCServer__start() {
    let mcserver = new_mcserver::<MyConfig>();

    MCServer::start(&mcserver, false).unwrap();
    if let Ok(mcserver) = mcserver.lock() {
        if let None = mcserver.minecraft_server {
            assert!(false, "Expected minecraft_server field to be filled.");
        }
        if let None = mcserver.main_thread {
            assert!(false, "Expected main_thread field to be filled.");
        }
        assert_eq!(mcserver.alive, true, "Expected mcserver field to be true.");
        if let MCServerStatus::Starting = mcserver.status {
        } else {
            assert!(false, "Expected status field to be MCServerStatus::Starting.");
        }; 
    } else {
        assert!(false, "Expected MCServer to not be corrupted.");
    }

    let status_closure = || -> MCServerStatus {
        return MCServer::get_lock_pure(&mcserver, true).unwrap().status.clone();
    };
    loop {
        if let MCServerStatus::Started = status_closure() {
            break;
        }
    }
    MCServer::stop(&mcserver, false).unwrap();
    cleanup();
}
#[test]
fn MCServer__stop() {
    let mcserver = new_mcserver::<MyConfig>();

    MCServer::start(&mcserver, false).unwrap();
    loop {
        if let Err(_) = MCServer::stop(&mcserver, false) {
        }
        else {
            break;
        }
    }
    if let Ok(mcserver) = mcserver.lock() {
        if let Some(_) = mcserver.minecraft_server {
            assert!(false, "Expected minecraft_server field to be empty.");
        }
        assert_eq!(mcserver.alive, false, "Expected alive field to be false.");
        if let Some(_) = mcserver.main_thread {
            assert!(false, "Expected main_thread field to be empty.");
        }
    } else {
        assert!(false, "Expected MCServer to not be corrupted.");
    }
    cleanup();
}
#[test]
fn MCServer__restart() {
    let mcserver = new_mcserver::<MyConfig>();

    MCServer::start(&mcserver, true).unwrap();
    MCServer::wait_for_start_confirm(&mcserver);
    loop {
        if let Err(_) = MCServer::restart(&mcserver) {
        }
        else {
            break;
        }
    }
    if let Ok(mcserver) = mcserver.lock() {
        if let None = mcserver.minecraft_server {
            assert!(false, "Expected minecraft_server field to be filled.");
        }
        if let None = mcserver.main_thread {
            assert!(false, "Expected main_thread field to be filled.");
        }
        assert_eq!(mcserver.alive, true, "Expected mcserver field to be true.");
        if let MCServerStatus::Started = mcserver.status {
        } else {
            assert!(false, "Expected status field to be MCServerStatus::Started.");
        };  
    } else {
        assert!(false, "Expected MCServer to not be corrupted.");
    }
    MCServer::stop(&mcserver, false).unwrap();
    cleanup();
}

#[test]
fn MCServer__send_input() {
    let mcserver = new_mcserver::<MyConfig>();
    let expected_string = " INFO]: Unknown command. Type \"/help\" for help.";

    MCServer::start(&mcserver, false).unwrap();
    loop {
        if let MCServerStatus::Started = mcserver.lock().unwrap().status {
            break;
        }
    }
    MCServer::send_input(&mcserver, "invalid_command");

    thread::sleep(*MyConfig::new().refresh_rate());

    let mut out = "".to_string();
    if let Err(_) = File::options().read(true).open("./logs/myMinecraftServer.txt").unwrap().read_to_string(&mut out) {}

    if !out.contains(expected_string) {
        assert!(false, "Expected `{expected_string}` in log. Found: {out}")
    }
    MCServer::stop(&mcserver, false).unwrap();
    cleanup();
}
#[test]
fn MCServer__save_output() {
    let mcserver = new_mcserver_no_download::<MyConfig>();
    let mcserver_lock = MCServer::get_lock(&mcserver);

    MCServer::save_output("Test line", &mcserver_lock);

    let mut out = "".to_string();
    if let Err(_) = File::options().read(true).open("./logs/myMinecraftServer.txt").unwrap().read_to_string(&mut out) {}

    assert_eq!(out, "Test line\n")
}

#[test]
fn MCServer__get_stdout_pipe() {
    let mcserver = new_mcserver::<MyConfig>();
    MCServer::start(&mcserver, false).unwrap();

    MCServer::get_stdout_pipe(&mut MCServer::get_lock_pure(&mcserver, true).unwrap()).unwrap();
    cleanup();
}
#[test]
fn MCServer__check_started() {
    let mcserver = new_mcserver_no_download::<MyConfig>();

    if !MCServer::check_started("[13:40:24 INFO]: Done (10.619s)! For help, type \"help\"", Instant::now(), &mcserver, false).unwrap() {
        assert!(false, "Expected function to detect a 'start'");
    }
    if let Ok(mcserver) = mcserver.lock() {
        if let MCServerStatus::Started = mcserver.status {
        } else {
            assert!(false, "Expected status field to be MCServerStatus::Started.");
        };
    } else {
        assert!(false, "Expected MCServer to not be corrupted.");
    }
    cleanup();
}
#[test]
fn MCServer__check_player_activity__connect() {
    let mcserver = new_mcserver_no_download::<MyConfig>();

    MCServer::check_player_activity("[13:53:51 INFO]: Gooxey joined the game", &mcserver).unwrap();
    if let Ok(mcserver) = mcserver.lock() {
        assert_eq!(mcserver.players, vec!["Gooxey".to_owned()], "Expected Gooxey to be in the players list.");
    } else {
        assert!(false, "Expected MCServer to not be corrupted.");
    }
    cleanup();
}
#[test]
fn MCServer__check_player_activity__disconnect() {
    let mcserver = new_mcserver_no_download::<MyConfig>();
    MCServer::check_player_activity("[13:53:51 INFO]: Gooxey joined the game", &mcserver).unwrap();

    MCServer::check_player_activity("[13:53:51 INFO]: Gooxey left the game", &mcserver).unwrap();
    if let Ok(mcserver) = mcserver.lock() {
        let vec: Vec<String> = vec![];
        assert_eq!(mcserver.players, vec, "Expected no one to be in the players list.");
    } else {
        assert!(false, "Expected MCServer to not be corrupted.");
    }
    cleanup();
}
#[test]
fn MCServer__agree_to_eula__already_accepted() {
    let mcserver = new_mcserver_no_download::<MyConfig>();
    let mcserver_lock = mcserver.lock().unwrap();

    fs::create_dir_all("./servers/myMinecraftServer").unwrap();
    let mut file = File::options().write(true).create_new(true).open("./servers/myMinecraftServer/eula.txt").unwrap();
    let text = "eula=true";
    io::copy(&mut text.as_bytes(), &mut file).unwrap();

    MCServer::agree_to_eula(&mcserver_lock).unwrap();

    let mut eula_txt = "".to_string();
    if let Err(_) = File::options().read(true).open(mcserver_lock.path.clone() + "/eula.txt").unwrap().read_to_string(&mut eula_txt) { }

    if !eula_txt.contains("eula=true") {
        assert!(false, "the eula text has been changed")
    }
    cleanup();
}
#[test]
fn MCServer__agree_to_eula__already_not_accepted() {
    let mcserver = new_mcserver_no_download::<MyConfig>();
    let mcserver_lock = mcserver.lock().unwrap();

    fs::create_dir_all("./servers/myMinecraftServer").unwrap();
    let mut file = File::options().write(true).create_new(true).open("./servers/myMinecraftServer/eula.txt").unwrap();
    let text = "eula=false";
    io::copy(&mut text.as_bytes(), &mut file).unwrap();

    MCServer::agree_to_eula(&mcserver_lock).unwrap();

    let mut eula_txt = "".to_string();
    if let Err(_) = File::options().read(true).open(mcserver_lock.path.clone() + "/eula.txt").unwrap().read_to_string(&mut eula_txt) { }

    if !eula_txt.contains("eula=true") {
        assert!(false, "the eula text is still false")
    }
    cleanup();
}
#[test]
fn MCServer__agree_to_eula__not_existing() {
    let mcserver = new_mcserver_no_download::<MyConfig>();
    let mcserver_lock = mcserver.lock().unwrap();

    fs::create_dir_all("./servers/myMinecraftServer").unwrap();

    MCServer::agree_to_eula(&mcserver_lock).unwrap();

    let mut eula_txt = "".to_string();
    if let Err(_) = File::options().read(true).open(mcserver_lock.path.clone() + "/eula.txt").unwrap().read_to_string(&mut eula_txt) { }

    if !eula_txt.contains("eula=true") {
        assert!(false, "the eula text is still false")
    }
    cleanup();
}