#![allow(non_snake_case)]
#![cfg(test)]


use super::{*, mcserver::mcserver_status::MCServerStatus};
use crate::test_functions::*;


fn test_start() -> Arc<Mutex<MCServerManager<Config>>> {
    generate_server_list();
    generate_MCServerManager()
}
fn generate_server_list() {
    cleanup();
    let content = "{
        \"0\": {
            \"name\": \"myMinecraftServer\",
            \"arg\": \"-jar purpur-1.19.3-1876.jar nogui\",
            \"type\": \"purpur\" 
        }
    }";

    fs::create_dir("servers").unwrap();
    let mut server_list_file = File::options().write(true).create_new(true).open("servers/server_list.json").unwrap();
    io::copy(&mut content.as_bytes(), &mut server_list_file).unwrap();
}
fn generate_MCServerManager() -> Arc<Mutex<MCServerManager<Config>>> {
    download_minecraft_server();

    MCServerManager::new(Arc::new(Config::new())).unwrap()
}
fn download_minecraft_server() {
    let mut resp = reqwest::blocking::get("https://api.purpurmc.org/v2/purpur/1.19.3/1876/download").expect("An error occurred while downloading the Minecraft server");
    fs::create_dir_all("servers/myMinecraftServer").expect("An error occurred while creating the servers dir");
    let mut out = File::create("servers/myMinecraftServer/purpur-1.19.3-1876.jar").expect("failed to create file `purpur-1.19.3-1876.jar`");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
}

// the following two functions will also test `get_server_parameter` and `generate_valid_server_list_file`
#[test]
fn MCServerManager__load_mcserver_list_valid_file() {
    let mcserver_manager = test_start();

    MCServerManager::load_mcserver_list(&mcserver_manager).unwrap();

    let mcserver_list = &MCServerManager::get_lock(&mcserver_manager).mcserver_list;

    assert_eq!(mcserver_list.len(), 1, "The function should only have captured one server.");
    cleanup();
}
#[test]
fn MCServerManager__load_mcserver_list_invalid_file() {
    cleanup();
    let content = "{
        \"0\": {
            \"name\": \"myMinecraftServer\",
            \"arg\": \"-jar purpur-1.19.3-1876.jar -Xmx4G nogui\",
        }
    }";

    fs::create_dir("servers").unwrap();
    let mut server_list_file = File::options().write(true).create_new(true).open("servers/server_list.json").unwrap();
    io::copy(&mut content.as_bytes(), &mut server_list_file).unwrap();
    
    let mcserver_manager = Arc::new(Mutex::new(MCServerManager {
        mcserver_list: vec![],
        config: Arc::new(Config::new()),
        main_thread: None,
        alive: false
    }));


    MCServerManager::load_mcserver_list(&mcserver_manager).unwrap_err();

    File::options().write(true).create_new(true).open("servers/server_list.json").unwrap();
    File::options().write(true).create_new(true).open("servers/invalid_server_list.json").unwrap_err();
    File::options().write(true).create_new(true).open("servers/server_list_example.json").unwrap_err();
    cleanup();
}

#[test]
fn MCServerManager__get_mcserver() {
    let mcserver_manager = test_start();

    let mcserver = MCServerManager::get_mcserver(&mcserver_manager, "myMinecraftServer").unwrap();

    assert_eq!(MCServer::get_name(&mcserver).unwrap(), "myMinecraftServer");
    cleanup();
}
// set the `src/test_functions::AUTO_START` const to true to test the shutdown of the own machine in 1 min
// the `src/test_functions::AGREE_TO_EULA` const needs to be true
// the `src/test_functions::MCSERVER_RESTART_TIME` const needs to be true
#[test]
fn MCServerManager__main() { // this is a test for almost every function in the MCServerManager struct
    let mcserver_manager = test_start();

    MCServerManager::start(&mcserver_manager, true).unwrap();

    let mcserver = MCServerManager::get_mcserver(&mcserver_manager, "myMinecraftServer").unwrap();
    let start_time = Instant::now();
    loop {
        if let MCServerStatus::Restarting = MCServer::get_status(&mcserver).unwrap() {
            break;
        }
        if Instant::now() - start_time > Duration::new(100, 0) {
            assert!(false, "The MCServerManager took to long to restart.");
        }
    }
    loop {
        if let MCServerStatus::Started = MCServer::get_status(&mcserver).unwrap() {
            break;
        }
    }

    MCServerManager::stop(&mcserver_manager, true).unwrap();
}