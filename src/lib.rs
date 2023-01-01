//! Functions and structs used by applications in the MCManage Network.
//! 
//! ## Description
//! 
//! This is part of the [`MCManage`](https://github.com/Gooxey/MCManage.git) project, which tries to make it as easy as possible to create and manage your [`Minecraft servers`](https://www.minecraft.net).
//! 
//! ### Features
//! 
//! | Struct                        | Description                                                                                                                                                                                    |
//! |-------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | [`Message`](message::Message) | This struct represents the standard message, which is used to send commands or information between different applications in the [`MCManage network`](https://github.com/Gooxey/MCManage.git). |
//! 
//! | Enum                                                | Description                                                |
//! |-----------------------------------------------------|------------------------------------------------------------|
//! | [`MessageType`](message::message_type::MessageType) | This enum describes the type of message holding this enum. |
//! 
//! | Trait                      | Description                                                                   |
//! |----------------------------|-------------------------------------------------------------------------------|
//! | [`Config`](config::Config) | Every struct implementing this trait can be used as the application's config. |
//! 
//! | Error                                                   | Description                                                                                |
//! |---------------------------------------------------------|--------------------------------------------------------------------------------------------|
//! | [`MsgTypeError`](message::message_type::msg_type_error) | This error type gets used by the [`MessageType enum`](message::message_type::MessageType). |
//! 
//! | Function          | Description                                                                                                                                                                                      |
//! |-------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | [`log`](log::log) | This function can be used to print and save a given string to a file or the console. This can be done in a fancy mode (colored text) if enabled by the [`application's config`](config::Config). |
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