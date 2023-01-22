//! This module provides the [`Config trait`](Config). When this trait gets implemented by structs, they can be used as the application's config.

use std::net::SocketAddrV4;
use std::time::Duration;
use std::marker;


/// Every struct implementing this trait can be used as the application's config.
/// 
/// ## Methods
/// 
/// | Method                                                | Description                                                                                                                                    |
/// |-------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------|
/// | [`new()`](Config::new)                                | Create a new config instance.                                                                                                                  |
/// |                                                       |                                                                                                                                                |
/// | [`addr() -> &SocketAddrV4`](Config::new)              | Return the address of the machine running this application.                                                                                    |
/// | [`buffsize() -> &u32`](Config::buffsize)              | Return the size of the buffers created by this application. (If set too low, it can cause logs to only be partially transmitted.)              |
/// | [`refresh_rate() -> &Duration`](Config::refresh_rate) | Return the time the application waits between checks.                                                                                          |
/// | [`max_tries() -> &i32`](Config::max_tries)            | Return the maximum number of times an operation gets retried.                                                                                  |
/// | [`agree_to_eula() -> &bool`](Config::agree_to_eula)   | Return whether or not all EULAs for the Minecraft servers get accepted automatically. See the functions description for more information.      |
pub trait Config
where
    Self: marker::Send + marker::Sync + 'static
{   
    /// Create a new config instance.
    fn new() -> Self;
    /// Return the address of the machine running this application.
    fn addr(&self) -> &SocketAddrV4;
    /// Return the size of the buffers created by this application. (If set too low, it can cause logs to only be partially transmitted.)
    fn buffsize(&self) -> &u32;
    /// Return the time the application waits between checks.
    fn refresh_rate(&self) -> &Duration;
    /// Return the maximum number of times an operation gets retried.
    fn max_tries(&self) -> &i32;
    /// Return whether or not all EULAs for the Minecraft servers get accepted automatically. \
    /// The following line is copied from the vanilla Minecraft server's EULA. \
    /// ' By changing the setting below to TRUE you are indicating your agreement to our EULA <https://aka.ms/MinecraftEULA>. ' \
    /// In other words:, when this function returns true, you are agreeing to any EULA this application automatically accepts.
    fn agree_to_eula(&self) -> &bool;
}