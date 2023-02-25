//! This module provides the MCManageError which is used anywhere in the [`MCManage network`](https://github.com/Gooxey/MCManage.git).


use std::io;

use thiserror::Error;


/// This error type provides errors used anywhere in the [`MCManage network`](https://github.com/Gooxey/MCManage.git).
/// 
/// # Variants
/// 
/// | Variant                                                   | Description                                                                                        |
/// |-----------------------------------------------------------|----------------------------------------------------------------------------------------------------|
/// | [`CriticalError`](MCManageError::CriticalError)           | The function encountered a recoverable error and restarted the given struct.                       |
/// | [`FatalError`](MCManageError::FatalError)                 | The function encountered a non-recoverable error and reset the given struct.                       |
/// | [`UnwrapOnNone`](MCManageError::UnwrapOnNone)             | Called unwrap on a None value.                                                                     |
/// | [`InvalidFile`](MCManageError::InvalidFile)               | The function encountered an invalid file. See the function description for more information.       |
/// | [`NotFound`](MCManageError::NotFound)                     | The requested item could not be found.                                                             |
/// | [`AlreadyExecuted`](MCManageError::AlreadyExecuted)       | The function has already been executed.                                                            |
/// | [`CurrentlyExecuting`](MCManageError::CurrentlyExecuting) | The function is currently being executed by another thread.                                        |
/// | [`NotReady`](MCManageError::NotReady)                     | The function is not ready to be executed. Please try again later.                                  |
/// | [`NotStarted`](MCManageError::NotStarted)                 | The struct needs to be started before executing anything. Please execute the start function first. |
/// | [`IOError`](MCManageError::IOError)                       | An error of kind IOError occurred.                                                                 |
#[derive(Error, Debug)]
pub enum MCManageError {
    /// The function encountered a recoverable error and restarted the given struct.
    #[error("The function encountered a recoverable error and restarted the given struct.")]
    CriticalError,
    /// The function encountered a non-recoverable error and reset the given struct.
    #[error("The function encountered a non-recoverable error and reset the given struct.")]
    FatalError,
    /// Called unwrap on a None value.
    #[error("Called unwrap on a None value.")]
    UnwrapOnNone,
    /// The function encountered an invalid file. See the function description for more information.
    #[error("The function encountered an invalid file. See the function description for more information.")]
    InvalidFile,
    /// The requested item could not be found.
    #[error("The requested item could not be found.")]
    NotFound,
    /// The function has already been executed.
    #[error("The method has already been executed.")]
    AlreadyExecuted,
    /// The function is currently being executed by another thread.
    #[error("The method is currently being executed by another thread.")]
    CurrentlyExecuting,
    /// The function is not ready to be executed. Please try again later.
    #[error("The function is not ready to be executed. Please try again later.")]
    NotReady,
    /// The struct needs to be started before executing anything. Please execute the start function first.
    #[error("The struct needs to be started before executing anything. Please execute the start function first.")]
    NotStarted,
    /// An error of kind IOError occurred.
    #[error(transparent)]
    IOError(#[from] io::Error)
}