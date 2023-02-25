//! This module provides the [`Config struct`](Config), which is used all over the [`MCManage network`](https://github.com/Gooxey/MCManage.git) as the application's config.


use std::net::{SocketAddrV4, Ipv4Addr};
use std::time::Duration;


// The following line is copied from the Minecraft servers EULA
// By changing the setting below to TRUE you are indicating your agreement to our EULA (https://aka.ms/MinecraftEULA).
const AGREE_TO_EULA: bool = true;


/// This struct represents the config of applications in the [`MCManage network`](https://github.com/Gooxey/MCManage.git). \
/// 
/// # Fields
/// 
/// All of these fields can be accessed by their respective getter method. ( Example: 'addr' field => addr() )
/// 
/// | Field                                 | Description                                                                                                                                                                          |
/// |---------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
/// | `addr                 : SocketAddrV4` | The address of the machine running this application.                                                                                                                                 |
/// | `buffsize             : u32`          | The size of the buffers created by this application. (If set too low, it can cause logs to only be partially transmitted.)                                                           |
/// | `refresh_rate         : Duration`     | The time the application waits between checks.                                                                                                                                       |
/// | `max_tries            : i32`          | The maximum number of times an operation gets retried.                                                                                                                               |
/// | `agree_to_eula        : bool`         | Sets whether or not all EULAs for the Minecraft servers get accepted automatically. See the methods description for more information.                                                |
/// | `shutdown_time        : bool`         | If no player is playing on any server for that duration, the computer running this application gets shut down. If the value is 0, no shutdowns will be performed.                    |
/// | `mcserver_restart_time: Duration`     | The amount of time the [`MCServerManager`](crate::mcserver_manager::MCServerManager) should wait between restarts of the [`MCServers`](crate::mcserver_manager::mcserver::MCServer). |
pub struct Config {
    addr: SocketAddrV4,
    buffsize: u32,
    refresh_rate: Duration,
    max_tries: i32,
    agree_to_eula: bool,
    shutdown_time: Duration,
    mcserver_restart_time: Duration
}
impl Config {
    /// Create a new [`Config`] instance.
    pub fn new() -> Self {
        Self {
            addr: SocketAddrV4::new(Ipv4Addr::LOCALHOST, 25564),
            buffsize: 100000000,
            refresh_rate: Duration::new(0, 100000000),
            max_tries: 3,
            agree_to_eula: AGREE_TO_EULA,
            shutdown_time: Duration::new(0, 0),
            mcserver_restart_time: Duration::new(60, 0),
        }
    }
    /// Return the address of the machine running this application.
    pub fn addr(&self) -> &SocketAddrV4 {
        &self.addr
    }
    /// Return the size of the buffers created by this application. (If set too low, it can cause logs to only be partially transmitted.)
    pub fn buffsize(&self) -> &u32 {
        &self.buffsize
    }
    /// Return the time the application waits between checks.
    pub fn refresh_rate(&self) -> &Duration {
        &self.refresh_rate
    }
    /// Return the maximum number of times an operation gets retried.
    pub fn max_tries(&self) -> &i32 {
        &self.max_tries
    }
    /// Return whether or not all EULAs for the Minecraft servers get accepted automatically. \
    /// The following line is copied from the vanilla Minecraft server's EULA. \
    /// ' By changing the setting below to TRUE you are indicating your agreement to our EULA <https://aka.ms/MinecraftEULA>. ' \
    /// In other words:, when this function returns true, you are agreeing to any EULA this application automatically accepts.
    pub fn agree_to_eula(&self) -> &bool {
        &self.agree_to_eula
    }
    /// If no player is playing on any server for that duration, the computer running this application gets shut down. \
    /// If the value is 0, no shutdowns will be performed.
    pub fn shutdown_time(&self) -> &Duration {
        &self.shutdown_time
    }
    /// Return the amount of time the [`MCServerManager`](crate::mcserver_manager::MCServerManager) should wait between restarts of the [`MCServers`](crate::mcserver_manager::mcserver::MCServer). \
    /// If the value is 0, no restarts will be performed.
    pub fn mcserver_restart_time(&self) -> &Duration {
        &self.mcserver_restart_time
    }
}