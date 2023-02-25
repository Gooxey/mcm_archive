//! This module provides the [`MCServerType struct`](MCServerType), which is used to read the `config/mcserver_types.json` file and provide the [`MCServer`](super::MCServer) with strings
//! corresponding to different situations, like a player joining or leaving.


use std::io;
use std::path::Path;
use std::fs::{self, File};
use async_recursion::async_recursion;
use serde_json::Value;
use mcserver_types_default::MCSERVER_TYPES_DEFAULT;
use crate::log;
use crate::mcmanage_error::MCManageError;


mod tests;
pub mod mcserver_types_default;


/// With this struct, the [`MCServer`](super::MCServer) is able to interpret messages sent by a Minecraft server. \
/// To be exact, this struct is responsible for reading the `config/mcserver_types.json` file and providing the [`MCServer`](super::MCServer) with strings corresponding to 
/// different situations, like a player joining or leaving.
/// 
/// # Methods
/// 
/// | Method                                                                               | Description                                                  |
/// |--------------------------------------------------------------------------------------|--------------------------------------------------------------|
/// | [`new(...) -> Self`](MCServerType::new)                                              | Create a new [`MCServerType`](MCServerType).                 |
/// |                                                                                      |                                                              |
/// | [`get_started(...) -> Result<...>`](MCServerType::get_started)            	       | Get this Minecraft server types started message.             |
/// | [`get_player_joined(...) -> Result<...>`](MCServerType::get_player_joined)           | Get this Minecraft server types player joined message.       |
/// | [`get_player_left(...) -> Result<...>`](MCServerType::get_player_left)               | Get this Minecraft server types player left message.         |
/// | [`get_player_name_joined(...) -> Result<...>`](MCServerType::get_player_name_joined) | Get the name of the player that joined in the line provided. |
/// | [`get_player_name_left(...) -> Result<...>`](MCServerType::get_player_name_left)     | Get the name of the player that left in the line provided.   |
#[derive(Clone)]
pub struct MCServerType {
    server_type: String,
    parent: String // This struct is always held by a MCServer
}
impl MCServerType {
    /// Create a new [`MCServerType`].
    /// 
    /// # Parameters
    /// 
    /// | Parameter           | Description                                                                                                                                                                                      |
    /// |---------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
    /// | `server_type: &str` | To see all available options see the `config/mcserver_types.json` file. To see the standard options see the [`MCSERVER_TYPES_DEFAULT constant`](mcserver_types_default::MCSERVER_TYPES_DEFAULT). |
    /// | `parent: &str`      | The name of the [`MCServer`](super::MCServer) this [`MCServerType`] was meant for.                                                                                                               |
    pub fn new(server_type: &str, parent: &str) -> Self {
        Self {
            server_type: server_type.to_string(),
            parent: parent.to_string()
        }
    }

    /// Rename the invalid `config/mcserver_types.json` file to `config/invalid_mcserver_types.json` or something similar, and generate the default `config/mcserver_types.json` file,
    /// which is described in the [`MCSERVER_TYPES_DEFAULT constant`](mcserver_types_default::MCSERVER_TYPES_DEFAULT).
    fn generate_valid_mcserver_types_file(&self) {
        // rename the invalid file so that data will not get lost
        let mut invalid_file_name;
        let mut i = 0;
        let mut no_file_found = true;
        loop {
            if i == 0 {
                invalid_file_name = format!("config/invalid_mcserver_types.json");
            } else {
                invalid_file_name = format!("config/invalid_mcserver_types({}).json", i);
                no_file_found = false;
            }
            if !Path::new(&invalid_file_name).exists() {
                if let Err(_) = fs::rename("config/mcserver_types.json", &invalid_file_name) {
                    // the file does not exist -> the folder probably also not

                    if let Err(erro) = fs::create_dir("config") {
                        match erro.kind() {
                            io::ErrorKind::AlreadyExists => {}
                            _ => { panic!("This error occurred while trying to create the config folder: {erro}") }
                        }
                    }
                }
                break;
            } else {
                i += 1;
            }
        }
        if no_file_found {
            log!("warn", self.parent, "No `config/mcserver_types.json` file could be found. A new one will be generated.");
        } else {
            log!("warn", self.parent, "The mcserver types config at `config/mcserver_types.json` is invalid.");
            log!("warn", self.parent, "A new file will be generated, and the old one will be renamed to `{invalid_file_name}`.");
        }

        // generate the valid file
        let mut valid_mcserver_types_file = File::options().write(true).create_new(true).open("config/mcserver_types.json").unwrap(); // no error is expected, so we unwrap here
        io::copy(&mut MCSERVER_TYPES_DEFAULT.as_bytes(), &mut valid_mcserver_types_file).unwrap(); // no error is expected, so we unwrap here
    }
    
    /// Get a message from the `config/mcserver_types.json` file, which can be found under this MCServer's type ( vanilla, purpur, etc. ) and its
    /// identifier ( started, player_joined, etc. ). \
    /// \
    /// This method only works if the message to get is a single string. For messages containing multiple strings, use the
    /// [`get_message_vector method`](Self::get_message_vector).
    fn get_message(&self, identifier: &str) -> Result<Value, MCManageError> {
        // read a file given to a json object
        let mcserver_type_json: Value;
        if let Ok(file) = fs::read_to_string("config/mcserver_types.json") {
            if let Ok(json) = serde_json::from_str(&file) {
                mcserver_type_json = json;
            } else {
                self.generate_valid_mcserver_types_file();
                return Ok(Self::get_message(&self, identifier)?);
            }
        } else {
            self.generate_valid_mcserver_types_file();
            return Ok(Self::get_message(&self, identifier)?);
        }

        // get the json of a provided server type
        if let Some(server) = mcserver_type_json.get(&self.server_type) {
            if let Some(message) = server.get(identifier) {
                return Ok(message.to_owned())
            } else {
                self.generate_valid_mcserver_types_file();
                return Ok(Self::get_message(&self, identifier)?);
            }
        } else {
            return Err(MCManageError::NotFound);
        }
    }
    /// Get a message from the `config/mcserver_types.json` file, which can be found under this MCServer's type ( vanilla, purpur, etc. ) and its
    /// identifier ( started, player_joined, etc. ). \
    /// \
    /// This method is only useful if the message to be retrieved contains multiple strings. For messages containing a single string, use the
    /// [`get_message method`](Self::get_message).
    fn get_message_vector(&self, identifier: &str) -> Result<Vec<String>, MCManageError> {
        // convert the message got into a vector of strings and return it
        let mut final_vec: Vec<String> = vec![];
        if let Some (vec) = Self::get_message(&self, identifier)?.as_array() {
            for item in vec {
                if let Some(string) = item.as_str() {
                    final_vec.push(string.to_string());
                } else {
                    self.generate_valid_mcserver_types_file();
                    return Ok(Self::get_message_vector(&self, identifier)?);
                }
            }
            return Ok(final_vec);
        } else {
            self.generate_valid_mcserver_types_file();
            return Ok(Self::get_message_vector(&self, identifier)?);
        }
    }
    
    /// Get this Minecraft server types started message.
    pub async fn get_started(&self) -> Result<Vec<String>, MCManageError> {
        return Self::get_message_vector(&self, "started");
    }
    /// Get this Minecraft server types player joined message.
    pub async fn get_player_joined(&self) -> Result<Vec<String>, MCManageError> {
        return Self::get_message_vector(&self, "player_joined");
    }
    /// Get this Minecraft server types player left message.
    pub async fn get_player_left(&self) -> Result<Vec<String>, MCManageError> {
        return Self::get_message_vector(&self, "player_left");
    }

    /// Get the name of the player that joined in the line provided.
    #[async_recursion]
    pub async fn get_player_name_joined(&self, line: &str) -> Result<String, MCManageError> {
        let player_name_pos;
        if let Some(pos) = Self::get_message(&self, "player_name_joined_pos")?.as_u64() {
            player_name_pos = pos;
        } else {
            self.generate_valid_mcserver_types_file();
            return Ok(Self::get_player_name_joined(&self, line).await?);
        }

        let mut i: u64 = 0;
        let mut line_iter = line.split(" ").map(String::from);
        loop {
            if i >= player_name_pos {
                break;
            }

            line_iter.next();

            i += 1;
        }

        if let Some(player_name) = line_iter.next() {
            return Ok(player_name);
        } else {
            return Err(MCManageError::NotFound);
        }
    }
    /// Get the name of the player that left in the line provided.
    #[async_recursion]
    pub async fn get_player_name_left(&self, line: &str) -> Result<String, MCManageError> {
        let player_name_pos;
        if let Some(pos) = Self::get_message(&self, "player_name_left_pos")?.as_u64() {
            player_name_pos = pos;
        } else {
            self.generate_valid_mcserver_types_file();
            return Ok(Self::get_player_name_left(&self, line).await?);
        }

        let mut i: u64 = 0;
        let mut line_iter = line.split(" ").map(String::from);
        loop {
            if i >= player_name_pos {
                break;
            }

            line_iter.next();

            i += 1;
        }

        if let Some(player_name) = line_iter.next() {
            return Ok(player_name);
        } else {
            return Err(MCManageError::NotFound);
        }
    }
}