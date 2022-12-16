//! This module provides the [`Config struct`](Config) which represents the config of this applications.


mod tests;


use std::net::{SocketAddrV4, Ipv4Addr};
use std::time::Duration;


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
/// | [`runner_mac_addr() -> &Vec<String>`](Config::runner_mac_addr) | Return a list of all registered runners' Mac addresses. They are required for the machines running these runner applications to automatically boot up. |
/// | [`fancy_write() -> &bool`](Config::fancy_write)                | Return a bool used to control whether or not the [`log`](mcm_misc::log) printed should be colored. ( Some consoles do not support colored text. )      |
/// | [`max_tries() -> &i32`](Config::max_tries)                     | Return the maximum number of times an operation gets retried.                                                                                          |
pub struct Config {
    /// The address of the machine running this application.
    addr: SocketAddrV4,
    /// The buffer size for reading [`messages`](mcm_misc::message::Message) from the runner or client.
    buffsize: u32,
    /// The maximum time waited for a [`messages`](mcm_misc::message::Message) sent via external sockets or internal channels.
    refresh_rate: Duration,
    /// A list of all registered runners' Mac addresses. They are required for the machines running these runner applications to automatically boot up.
    runner_mac_addr: Vec<String>,
    /// A bool used to control whether or not the [`log`](mcm_misc::log) printed should be colored. ( Some consoles do not support colored text. )
    fancy_write: bool,
    /// The maximum number of times an operation gets retried.
    max_tries: i32
}
impl Config {
    /// Create a new [`Config`] instance. \
    /// This will currently set some predefined values for each field.
    /// 
    /// ## Usage
    /// 
    /// ```
    /// use crate::config::Config;
    /// 
    /// # fn main() {
    /// let myConfig = Config::new();
    /// # }
    /// ```
    /// 
    /// ## Predefined values
    /// 
    /// | Field                          | Value                                         |
    /// |--------------------------------|-----------------------------------------------|
    /// | `addr: SocketAddrV4`           | SocketAddrV4::new(Ipv4Addr::LOCALHOST, 25564) |
    /// | `buffsize: u32`                | 100000000                                     |
    /// | `refresh_rate: Duration`       | Duration::new(0, 100000000)                   |
    /// | `runner_mac_addr: Vec<String>` | vec!["44-8A-5B-8A-02-79".to_owned()]          |
    /// | `fancy_write: bool`            | true                                          |
    /// | `max_tries: i32`               | 3                                             |
    pub fn new() -> Self {
        Self {
            addr: SocketAddrV4::new(Ipv4Addr::LOCALHOST, 25564),
            buffsize: 100000000,
            refresh_rate: Duration::new(0, 100000000),
            runner_mac_addr: vec!["44-8A-5B-8A-02-79".to_owned()],
            fancy_write: true,
            max_tries: 3
        }
    }

    // Getter methods

    /// Return the address of the machine running this application.
    pub fn addr(&self) -> &SocketAddrV4 {
        &self.addr
    }
    /// Return the buffer size for reading [`messages`](mcm_misc::message::Message) from the runner or client.
    pub fn buffsize(&self) -> &u32 {
        &self.buffsize
    }
    /// Return the maximum time waited for a [`messages`](mcm_misc::message::Message) sent via external sockets or internal channels.
    pub fn refresh_rate(&self) -> &Duration {
        &self.refresh_rate
    }
    /// Return a list of all registered runners' Mac addresses. They are required for the machines running these runner applications to automatically boot up.
    pub fn runner_mac_addr(&self) -> &Vec<String> {
        &self.runner_mac_addr
    }
    /// Return a bool used to control whether or not the [`log`](mcm_misc::log) printed should be colored. ( Some consoles do not support colored text. )
    pub fn fancy_write(&self) -> &bool {
        &self.fancy_write
    }
    /// Return the maximum number of times an operation gets retried.
    pub fn max_tries(&self) -> &i32 {
        &self.max_tries
    }
}