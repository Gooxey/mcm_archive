//! This module provides the [`Config struct`](Config) which represents the config of this applications.


use std::net::{SocketAddrV4, Ipv4Addr};
use std::time::Duration;
use mcm_misc::config_trait::ConfigTrait;

const AGREE_TO_EULA: bool = false;


/// This struct represents the config of this application.
/// 
/// ## Methods
/// 
/// | Method                                                         | Description                                                                                                                                            |
/// |----------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------|
/// | [`new()`](Config::new)                                         | Create a new [`Config`] instance.                                                                                                                      |
/// |                                                                |                                                                                                                                                        |
/// | [`addr() -> &SocketAddrV4`](Config::new)                       | Return the address of the machine running this application.                                                                                            |
/// | [`buffsize() -> &u32`](Config::buffsize)                       | Return the buffer size for reading [`messages`](mcm_misc::message::Message) from the runner or client.                                                 |
/// | [`refresh_rate() -> &Duration`](Config::refresh_rate)          | Return the maximum time waited for a [`messages`](mcm_misc::message::Message) sent via external sockets or internal channels.                          |
/// | [`max_tries() -> &i32`](Config::max_tries)                     | Return the maximum number of times an operation gets retried.                                                                                          |
/// | [`agree_to_eula() -> &bool`](Config::max_tries)                | Return whether or not all EULAs for the Minecraft servers get accepted automatically.                                                                  |
pub struct Config {
    /// The address of the machine running this application.
    addr: SocketAddrV4,
    /// The buffer size for reading [`messages`](mcm_misc::message::Message) from the runner or client.
    buffsize: u32,
    /// The maximum time waited for a [`messages`](mcm_misc::message::Message) sent via external sockets or internal channels.
    refresh_rate: Duration,
    /// The maximum number of times an operation gets retried.
    max_tries: i32,
    /// Controls whether or not all EULAs for the Minecraft servers get accepted automatically.
    agree_to_eula: bool
}
impl ConfigTrait for Config {
    /// Create a new [`Config`] instance. \
    /// This will currently set some predefined values for each field.
    /// 
    /// ## Predefined values
    /// 
    /// | Field                          | Value                                         |
    /// |--------------------------------|-----------------------------------------------|
    /// | `addr: SocketAddrV4`           | SocketAddrV4::new(Ipv4Addr::LOCALHOST, 25564) |
    /// | `buffsize: u32`                | 100000000                                     |
    /// | `refresh_rate: Duration`       | Duration::new(0, 100000000)                   |
    /// | `max_tries: i32`               | 3                                             |
    /// | `agree_to_eula: bool`          | AGREE_TO_EULA                                 |
    fn new() -> Self {
        Self {
            addr: SocketAddrV4::new(Ipv4Addr::LOCALHOST, 25564),
            buffsize: 100000000,
            refresh_rate: Duration::new(0, 100000000),
            max_tries: 3,
            agree_to_eula: AGREE_TO_EULA
        }
    }

    // Getter methods

    /// Return the address of the machine running this application.
    fn addr(&self) -> &SocketAddrV4 {
        &self.addr
    }
    /// Return the buffer size for reading [`messages`](mcm_misc::message::Message) from the runner or client.
    fn buffsize(&self) -> &u32 {
        &self.buffsize
    }
    /// Return the maximum time waited for a [`messages`](mcm_misc::message::Message) sent via external sockets or internal channels.
    fn refresh_rate(&self) -> &Duration {
        &self.refresh_rate
    }
    /// Return the maximum number of times an operation gets retried.
    fn max_tries(&self) -> &i32 {
        &self.max_tries
    }
    /// Return whether or not all EULAs for the Minecraft servers get accepted automatically.
    /// The following line is copied from the vanilla Minecraft server's EULA.
    /// ' By changing the setting below to TRUE you are indicating your agreement to our EULA https://aka.ms/MinecraftEULA. '
    /// In other words:, when this function returns true, you are agreeing to any EULA this application automatically accepts.
    fn agree_to_eula(&self) -> &bool {
        &self.agree_to_eula
    }
}