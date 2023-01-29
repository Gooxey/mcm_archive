//! This module provides the [`MCServerError`], which is used by the [`MCServer`](super::MCServer).


use std::io;
use thiserror::Error;
use crate::mcmanage_error::MCManageError;

use super::mcserver_type::mcserver_type_error::MCServerTypeError;


/// Errors used by the [`MCServer struct`](super::MCServer).
/// 
/// ## Variants
/// 
/// | Variant                                                                      | Description                                                                                                     |
/// |------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------|
/// | [`FailedCommandSpawn(String, io::Error)`](MCServerError::FailedCommandSpawn) | Failed to spawn the assigned Minecraft server. The MCServer has been reset.                                     |
/// | [`CriticalError`](MCServerError::CriticalError)                              | The MCServer got restarted to fix a critical error. The function returning this needs to be called again.       |
/// | [`FatalError`](MCServerError::FatalError)                                    | The MCServer encountered a fatal error and was reset.                                                           |
/// | [`NotStarted`](MCServerError::NotStarted)                                    | The function called cannot be executed since the MCServer has not yet started.                                  |
/// | [`TypeError(MCServerTypeError)`](MCServerError::TypeError)                   | An error produced by the [`MCServerType struct`](super::mcserver_type::mcserver_type_error::MCServerTypeError). |
#[derive(Error, Debug)]
pub enum MCServerError {
    /// Failed to spawn the assigned Minecraft server. The MCServer has been reset.
    #[error("Failed to spawn the assigned Minecraft server {0}. The MCServer has been reset. Error: {1}")]
    FailedCommandSpawn(String, io::Error),
    /// The MCServer got restarted to fix a critical error. The function returning this needs to be called again.
    #[error("The MCServer got restarted to fix a critical error. The function returning this needs to be called again.")]
    CriticalError,
    /// The MCServer encountered a fatal error and was reset.
    #[error("The MCServer encountered a fatal error and was reset.")]
    FatalError,
    /// The function called cannot be executed since the MCServer has not yet started.
    #[error("The function called cannot be executed since the MCServer has not yet started.")]
    NotStarted,
    /// An error produced by the [`MCServerType struct`](super::mcserver_type::mcserver_type_error::MCServerTypeError)
    #[error(transparent)]
    TypeError(#[from] MCServerTypeError),
    #[error(transparent)]
    MCManageError(#[from] MCManageError)
}