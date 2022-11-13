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
//! |                  | [`MismatchedResult`](ChannelError::MismatchedResult)             | The wrong result was received!                                                                        |

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
/// | [`MismatchedResult`](ChannelError::MismatchedResult)             | The wrong result was received! 
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
    /// A fatal error occurred. The communication network had to be restarted. Error: One mutex was poisoned!
    FatalError,
    /// The wrong result was received!
    MismatchedResult
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
                write!(f, "A Fatal error occurred! The communication network had to be restart. Error: One mutex was poisoned!")
            }
            ChannelError::MismatchedResult => {
                write!(f, "The wrong result was received!")
            }
        }
    }
}
impl Error for ChannelError {}