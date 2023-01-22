#![allow(non_snake_case)]
#![cfg(test)]


use std::io::Read;

use super::*;
use crate::test_functions::*;


fn start_test() -> MCServerType {
    cleanup();
    MCServerType::new("purpur")
}


#[test]
fn MCServerType__new() {
    let my_mcserver_type = start_test();

    assert_eq!(my_mcserver_type.server_type, "purpur".to_string());

    cleanup();
}

#[test]
fn MCServerType__generate_valid_mcserver_types_file__no_file_there() {
    cleanup();
    MCServerType::generate_valid_mcserver_types_file();

    let mut file = File::options().read(true).open("config/mcserver_types.json").unwrap();
    let mut buf = "".to_string();

    file.read_to_string(&mut buf).unwrap();

    assert_eq!(buf, MCSERVER_TYPES_DEFAULT);

    cleanup();
}
#[test]
fn MCServerType__generate_valid_mcserver_types_file__one_file_there() {
    cleanup();
    fs::create_dir("config").unwrap();
    let mut invalid_mcserver_types_file_1 = File::options().write(true).create_new(true).open("config/mcserver_types.json").unwrap();
    io::copy(&mut "Invalid content 1".as_bytes(), &mut invalid_mcserver_types_file_1).unwrap();

    MCServerType::generate_valid_mcserver_types_file();

    let mut file_0 = File::options().read(true).open("config/mcserver_types.json").unwrap();
    let mut file_1 = File::options().read(true).open("config/invalid_mcserver_types.json").unwrap();
    
    let mut buf_0 = "".to_string();
    let mut buf_1 = "".to_string();

    file_0.read_to_string(&mut buf_0).unwrap();
    file_1.read_to_string(&mut buf_1).unwrap();

    assert_eq!(buf_0, MCSERVER_TYPES_DEFAULT);
    assert_eq!(buf_1, "Invalid content 1");

    cleanup();
}
#[test]
fn MCServerType__generate_valid_mcserver_types_file__two_files_there() {
    cleanup();
    fs::create_dir("config").unwrap();
    let mut invalid_mcserver_types_file_1 = File::options().write(true).create_new(true).open("config/mcserver_types.json").unwrap();
    let mut invalid_mcserver_types_file_2 = File::options().write(true).create_new(true).open("config/invalid_mcserver_types.json").unwrap();
    io::copy(&mut "Invalid content 1".as_bytes(), &mut invalid_mcserver_types_file_1).unwrap();
    io::copy(&mut "Invalid content 2".as_bytes(), &mut invalid_mcserver_types_file_2).unwrap();

    MCServerType::generate_valid_mcserver_types_file();

    let mut file_0 = File::options().read(true).open("config/mcserver_types.json").unwrap();
    let mut file_1 = File::options().read(true).open("config/invalid_mcserver_types.json").unwrap();
    let mut file_2 = File::options().read(true).open("config/invalid_mcserver_types(1).json").unwrap();
    
    let mut buf_0 = "".to_string();
    let mut buf_1 = "".to_string();
    let mut buf_2 = "".to_string();

    file_0.read_to_string(&mut buf_0).unwrap();
    file_1.read_to_string(&mut buf_1).unwrap();
    file_2.read_to_string(&mut buf_2).unwrap();

    assert_eq!(buf_0, MCSERVER_TYPES_DEFAULT);
    assert_eq!(buf_1, "Invalid content 2");
    assert_eq!(buf_2, "Invalid content 1");
    
    cleanup();
}

// get_message and get_message_vector got both indirectly tested by the tests below

#[test]
fn MCServerType__get_started() {
    let my_mcserver_type = start_test();
    
    assert_eq!(my_mcserver_type.get_started().unwrap(), [" INFO]: Done (", ")! For help, type \"help\""]);

    cleanup();
}
#[test]
fn MCServerType__get_player_joined() {
    let my_mcserver_type = start_test();
    
    assert_eq!(my_mcserver_type.get_player_joined().unwrap()[0], " joined the game");

    cleanup();
}
#[test]
fn MCServerType__get_player_left() {
    let my_mcserver_type = start_test();
    
    assert_eq!(my_mcserver_type.get_player_left().unwrap()[0], "left the game");

    cleanup();
}

#[test]
fn MCServerType__get_player_name_joined() {
    let my_mcserver_type = start_test();

    let name = my_mcserver_type.get_player_name_joined("[11:48:16] [Server thread/INFO]: Gooxey joined the game").unwrap();
    
    assert_eq!(name, "Gooxey");

    cleanup();
}
#[test]
fn MCServerType__get_player_name_left() {
    let my_mcserver_type = start_test();

    let name = my_mcserver_type.get_player_name_left("[12:30:46] [Server thread/INFO]: Gooxey left the game").unwrap();
    
    assert_eq!(name, "Gooxey");

    cleanup();
}   