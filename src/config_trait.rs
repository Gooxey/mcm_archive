//! This module provides the [`ConfigTrait trait`](ConfigTrait). When this trait gets implemented by structs, they can be used as the application's config.

use std::net::SocketAddrV4;
use std::time::Duration;
use std::marker;


/// Every struct implementing this trait can be used as the application's config.
/// 
/// ## Methods
/// 
/// | Method                                                                         | Description                                                                                                                                                                                 |
/// |--------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
/// | [`new()`](ConfigTrait::new)                                                    | Create a new config instance.                                                                                                                                                               |
/// |                                                                                |                                                                                                                                                                                             |
/// | [`addr() -> &SocketAddrV4`](ConfigTrait::new)                                  | Return the address of the machine running this application.                                                                                                                                 |
/// | [`buffsize() -> &u32`](ConfigTrait::buffsize)                                  | Return the size of the buffers created by this application. (If set too low, it can cause logs to only be partially transmitted.)                                                           |
/// | [`refresh_rate() -> &Duration`](ConfigTrait::refresh_rate)                     | Return the time the application waits between checks.                                                                                                                                       |
/// | [`max_tries() -> &i32`](ConfigTrait::max_tries)                                | Return the maximum number of times an operation gets retried.                                                                                                                               |
/// | [`agree_to_eula() -> &bool`](ConfigTrait::agree_to_eula)                       | Return whether or not all EULAs for the Minecraft servers get accepted automatically. See the functions description for more information.                                                   |
/// | [`shutdown_time() -> &bool`](ConfigTrait::shutdown_time)                       | If no player is playing on any server for that duration, the computer running this application gets shut down. If the value is 0, no shutdowns will be performed.                           |
/// | [`mcserver_restart_time() -> &Duration`](ConfigTrait::mcserver_restart_time)   | Return the amount of time the [`MCServerManager`](crate::mcserver_manager::MCServerManager) should wait between restarts of the [`MCServers`](crate::mcserver_manager::mcserver::MCServer). |
pub trait ConfigTrait
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
    /// If no player is playing on any server for that duration, the computer running this application gets shut down. \
    /// If the value is 0, no shutdowns will be performed.
    fn shutdown_time(&self) -> &Duration;
    /// Return the amount of time the [`MCServerManager`](crate::mcserver_manager::MCServerManager) should wait between restarts of the [`MCServers`](crate::mcserver_manager::mcserver::MCServer). \
    /// If the value is 0, no restarts will be performed.
    fn mcserver_restart_time(&self) -> &Duration;
}