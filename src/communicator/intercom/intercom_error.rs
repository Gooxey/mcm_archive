//! This module provides the [`InterComError`], which is used by the [`InterCom`](super::InterCom).


use thiserror::Error;
use mcm_misc::mcmanage_error::MCManageError;


/// Errors used by the [`InterCom`](super::InterCom).
/// 
/// ## Variants
/// 
/// | Variant                                                           | Description                                                                                           |
/// |-------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------|
/// | [`DesyncedChannelStorage`](InterComError::DesyncedChannelStorage) | The ID is available in the ID storage but has been taken in the channel storage!                      |
/// | [`IDNotFound`](InterComError::IDNotFound)                         | The given channel_id could not be found in both channel_lists!                                        |
/// | [`InvalidType`](InterComError::InvalidType)                       | The given channel_type is not supported!                                                              |
#[derive(Error, Debug)]
pub enum InterComError {
    /// The ID is available in the ID storage but has been taken in the channel storage! 
    /// 
    /// # Parameter
    /// 
    /// `String` => The handlers id throwing this error.
    #[error("The ID is available in the ID storage but has been taken in the channel storage!")]
    DesyncedChannelStorage(String),
    /// The given channel_id could not be found in both channel_lists!
    /// 
    /// # Parameter
    /// 
    /// `String` => The handlers id throwing this error.
    #[error("The given channel_id could not be found in both channel_lists!")]
    IDNotFound(String),
    /// The given channel_type is not supported!
    #[error("The given channel_type is not supported!")]
    InvalidType(char),
    #[error(transparent)]
    MCManageError(#[from] MCManageError)
}