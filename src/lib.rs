//! Functions and structs used by applications in the MCManage Network.
//! 
//! ## Description
//! This is part of the [`MCManage`](https://github.com/Gooxey/MCManage.git) project, which tries to make it as easy as possible to create and manage your [Minecraft](https://www.minecraft.net) servers.
//! 
//! ## Features
//! 
//! | Struct                             | Description                                                                                                                                        |
//! |------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|
//! | [Message](crate::message::Message) | This struct represents the standard message, which is used to send commands or information between different applications in the MCManage network. |
//! 
//! | Function               | Description                                                                                                                                                        |
//! |------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | [log](crate::log::log) | This function prints and/or saves a given string to the console or log file. A fancy mode will also be used if configured in the configuration of the application. |

pub mod log;
pub mod message;
pub mod config;