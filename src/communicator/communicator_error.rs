//! This module provides the [`CommunicatorError`], which is used by the [`Communicator`](super::Communicator).


use mcm_misc::mcmanage_error::MCManageError;
use super::intercom::intercom_error::InterComError;
use thiserror::Error;



/// Errors used by the [`Communicator module`](crate::communicator).
/// 
/// ## Variants
/// 
/// | Variant                                                 | Description                                                                                                  |
/// |---------------------------------------------------------|--------------------------------------------------------------------------------------------------------------|
/// | [`FailedBind`](CommunicatorError::FailedBind)           | The Communicator failed to start its TCPServer!                                                              |
/// | [`ConnectionError`](CommunicatorError::ConnectionError) | A fatal error occurred. The connection had to be closed.                                                     |
/// | [`RestartError`](CommunicatorError::RestartError)       | The maximum number of restart attempts has been reached. The Communicator will no longer attempt to restart. |
#[derive(Error, Debug)]
pub enum CommunicatorError {
    /// The Communicator failed to start its TCPServer!
    #[error("The Communicator failed to start its TCPServer!")]
    FailedBind,
    /// A fatal error occurred. The connection had to be closed.
    #[error("A fatal error occurred. The connection had to be closed.")]
    ConnectionError,
    /// The maximum number of restart attempts has been reached. The Communicator will no longer attempt to restart.
    #[error("The maximum number of restart attempts has been reached. The Communicator will no longer attempt to restart.")]
    RestartError,
    #[error(transparent)]
    InterComError(#[from] InterComError),
    #[error(transparent)]
    MCManageError(#[from] MCManageError)
}