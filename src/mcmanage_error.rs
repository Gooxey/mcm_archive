//! This module provides the MCManageError which is used almost anywhere in the [`MCManage network`](https://github.com/Gooxey/MCManage.git).


use thiserror::Error;


/// This error type provides errors used almost anywhere in the [`MCManage network`](https://github.com/Gooxey/MCManage.git).
/// 
/// ## Variants
/// 
/// | Variant                                         | Description                                                                    |
/// |-------------------------------------------------|--------------------------------------------------------------------------------|
/// | [`CriticalError`](MCManageError::CriticalError) | The struct got restarted to fix a critical error.                              |
/// | [`FatalError`](MCManageError::FatalError)       | The struct encountered a fatal error and needs to be reset.                    |
/// | [`NotReady`](MCManageError::NotReady)           | The struct is not ready to perform a certain function. Please try again later. |
#[derive(Error, Debug)]
pub enum MCManageError {
    /// The struct got restarted to fix a critical error.\ 
    /// The struct's thread receiving this error needs to exit immediately after restarting the struct.
    #[error("The struct got restarted to fix a critical error.")]
    CriticalError,
    /// The struct encountered a fatal error and needs to be reset. \
    /// The struct's thread receiving this error needs to exit immediately after resetting the struct.
    #[error("The struct encountered a fatal error and needs to be reset.")]
    FatalError,
    /// The struct is not ready to perform a certain function. Please try again later.
    #[error("The struct is not ready to perform a certain function. Please try again later.")]
    NotReady
}