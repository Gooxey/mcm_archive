//! This module provides various custom errors used in this application.
//! 
//! ## Errors
//! 
//! | Group            | Variant                                                          | Description                                                                                           |
//! |------------------|------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------|
//! | [`ChannelError`] | [`DesyncedChannelStorage`](ChannelError::DesyncedChannelStorage) | The ID is available in the ID storage but has been taken in the channel storage!                      |
//! |                  | [`IDNotFound`](ChannelError::IDNotFound)                         | The given channel_id could not be found in both channel_lists!                                        |
//! |                  | [`InvalidType`](ChannelError::InvalidType)                       | The given channel_type is not supported!                                                              |
//! |                  | [`FatalError`](ChannelError::FatalError)                         | A fatal error occurred. The communication network had to be restarted. Error: One mutex was poisoned! |

use std::{error::{Error}, fmt};

/// Errors used by the [`Communicator module`](crate::communicator).
/// 
/// ## Variants
/// 
/// | Variant                                                          | Description                                                                                           |
/// |------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------|
/// | [`DesyncedChannelStorage`](ChannelError::DesyncedChannelStorage) | The ID is available in the ID storage but has been taken in the channel storage!                      |
/// | [`IDNotFound`](ChannelError::IDNotFound)                         | The given channel_id could not be found in both channel_lists!                                        |
/// | [`InvalidType`](ChannelError::InvalidType)                       | The given channel_type is not supported!                                                              |
/// | [`FatalError`](ChannelError::FatalError)                         | A fatal error occurred. The communication network had to be restarted. Error: One mutex was poisoned! |
#[derive(Debug)]
pub enum ChannelError {
    /// The ID is available in the ID storage but has been taken in the channel storage! 
    /// 
    /// # Parameter
    /// 
    /// `String` => The handlers id throwing this error.
    DesyncedChannelStorage(String),
    /// The given channel_id could not be found in both channel_lists!
    /// 
    /// # Parameter
    /// 
    /// `String` => The handlers id throwing this error.
    IDNotFound(String),
    /// The given channel_type is not supported!
    InvalidType(char),
    /// A fatal error occurred. The Communicator had to be restarted.
    FatalError
}
impl fmt::Display for ChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChannelError::DesyncedChannelStorage(channel_id) => {
                write!(f, "The ID `{channel_id}` is free in the ID storage but taken in the channel storage!")
            }
            ChannelError::IDNotFound(channel_id) => {
                write!(f, "The channel_id `{channel_id}` could not be found in the handler_send and handler_recv lists!")
            }
            ChannelError::InvalidType(channel_type) => {
                write!(f, "The given channel_type `{channel_type}` is not supported! Please use one of the following types: `r`, `c`")
            }
            ChannelError::FatalError => {
                write!(f, "A Fatal error occurred! The Communicator had to be restart.")
            }
        }
    }
}
impl Error for ChannelError {}


/// Errors used by the [`Communicator module`](crate::communicator).
/// 
/// ## Variants
/// 
/// | Variant                                                 | Description                                                   |
/// |---------------------------------------------------------|---------------------------------------------------------------|
/// | [`FailedBind`](CommunicatorError::FailedBind)           | The Communicator failed to start its TCPServer!               |
/// | [`FatalError`](CommunicatorError::FatalError)           | A fatal error occurred. The Communicator had to be restarted. |
/// | [`ConnectionError`](CommunicatorError::ConnectionError) | A fatal error occurred. The connection will be closed.        |
#[derive(Debug)]
#[derive(PartialEq)]
pub enum CommunicatorError {
    /// The Communicator failed to start its TCPServer!
    FailedBind(),
    /// A fatal error occurred. The Communicator had to be restarted.
    FatalError(),
    /// A fatal error occurred.
    ConnectionError(),
    /// The maximum number of restart attempts has been reached. The Communicator will no longer attempt to restart.
    RestartError()
}
impl fmt::Display for CommunicatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommunicatorError::FailedBind() => {
                write!(f, "The Communicator failed to start its TCPServer!")
            }
            CommunicatorError::FatalError() => {
                write!(f, "A fatal error occurred.")
            }
            CommunicatorError::ConnectionError() => {
                write!(f, "A fatal error occurred. The connection had to be closed.")
            }
            CommunicatorError::RestartError() => {
                write!(f, "The maximum number of restart attempts has been reached. The Communicator will no longer attempt to restart.")
            }
        }
    }
}
impl Error for CommunicatorError {}