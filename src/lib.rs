//! Functions and structs used by applications in the [MCManage Network](https://github.com/Gooxey/MCManage.git).
//! 
//! ## Description
//! 
//! This is part of the [`MCManage`](https://github.com/Gooxey/MCManage.git) project, which tries to make it as easy as possible to create and manage your [`Minecraft servers`](https://www.minecraft.net).
//! 
//! ### Message
//! 
//! | Struct                        | Description                                                                                                                                                                                    |
//! |-------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | [`Message`](message::Message) | This struct represents the standard message, which is used to send commands or information between different applications in the [`MCManage network`](https://github.com/Gooxey/MCManage.git). |
//! 
//! | Enum                                                | Description                                                |
//! |-----------------------------------------------------|------------------------------------------------------------|
//! | [`MessageType`](message::message_type::MessageType) | This enum describes the type of message holding this enum. |
//! 
//! | Error                                                   | Description                                                                                |
//! |---------------------------------------------------------|--------------------------------------------------------------------------------------------|
//! | [`MsgTypeError`](message::message_type::msg_type_error) | This error type gets used by the [`MessageType enum`](message::message_type::MessageType). |
//! 
//! 
//!  ### Config
//! 
//! | Trait                      | Description                                                                   |
//! |----------------------------|-------------------------------------------------------------------------------|
//! | [`Config`](config::Config) | Every struct implementing this trait can be used as the application's config. |
//! 
//! 
//! ### Log
//! 
//! | Macros         | Description                                                                       |
//! |----------------|-----------------------------------------------------------------------------------|
//! | [`log!`]       | This macro can be used to print a given string to the console.                    |
//! | [`log_print!`] | This macro can be used to print and save a given string to a file or the console. |
//! 
//! 
//! ### MCServer
//! 
//! | Struct                                                  | Description                                                                                                    |
//! |---------------------------------------------------------|----------------------------------------------------------------------------------------------------------------|
//! | [`MCServer`](mcserver::MCServer)                        | This struct represents an API for one Minecraft server, which got assigned with the initiation of this struct. |
//! | [`MCServerType`](mcserver::mcserver_type::MCServerType) | With this struct, the MCServer is able to interpret messages sent by a Minecraft server.                       |
//! 
//! | Enum                                                          | Description                                                         |
//! |---------------------------------------------------------------|---------------------------------------------------------------------|
//! | [`MCServerStatus`](mcserver::mcserver_status::MCServerStatus) | This enum represents the [`MCServer's`](mcserver::MCServer) status. |
//! 
//! | Error                                                                                  | Description                                                                        |
//! |----------------------------------------------------------------------------------------|------------------------------------------------------------------------------------|
//! | [`MCServerError`](mcserver::mcserver_error::MCServerError)                             | Errors used by the [`MCServer`](mcserver::MCServer) struct.                        |
//! | [`MCServerTypeError`](mcserver::mcserver_type::mcserver_type_error::MCServerTypeError) | Errors used by the [`MCServerType`](mcserver::mcserver_type::MCServerType) struct. |
//! 
//! | Constant                                                                                            | Description                                                                       |
//! |-----------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------|
//! | [`MCSERVER_TYPES_DEFAULT`](mcserver::mcserver_type::mcserver_types_default::MCSERVER_TYPES_DEFAULT) | This constant represents the default text in the config/mcserver_types.json file. |
//!  
//! ## Installation
//! 
//! Add the dependency to the `cargo.toml` file:
//! ```
//! [dependencies]
//! mcm_misc = { git = "https://github.com/Gooxey/mcm_misc.git", version = "X.Y.Z" }
//!     or
//! mcm_misc = { path = "/path/to/mcm_misc/" }
//! ```

pub mod log;
pub mod message;
pub mod mcserver;
pub mod config;

mod test_functions;