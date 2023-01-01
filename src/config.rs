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
/// | [`fancy_write() -> &bool`](Config::fancy_write)       | Return a bool used to control whether or not the [`log`](crate::log) printed should be colored. ( Some consoles do not support colored text. ) |
/// | [`max_tries() -> &i32`](Config::max_tries)            | Return the maximum number of times an operation gets retried.                                                                                  |
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
    /// Return a bool used to control whether or not the [`log`](crate::log) printed should be colored. ( Some consoles do not support colored text. )
    fn fancy_write(&self) -> &bool;
    /// Return the maximum number of times an operation gets retried.
    fn max_tries(&self) -> &i32;
}