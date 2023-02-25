//! This module provides various useful methods for the implementation of the [`ConcurrentClass`] trait.


use std::sync::Arc;

use tokio::time::sleep;

use crate::mcmanage_error::MCManageError;

use super::ConcurrentClass;
use super::status::Status;


/// Check if the [`impl_start`](ConcurrentClass::impl_start) method is allowed to be executed. \
/// This function will also set the status of the given class to the right value.
/// 
/// # Returns
/// 
/// | Return                                | Description                                               |
/// |---------------------------------------|-----------------------------------------------------------|
/// | [`Ok(())`]                            | The method can be executed immediately.                   |
/// | [`MCManageError::AlreadyExecuted`]    | The method has already been executed.                     |
/// | [`MCManageError::CurrentlyExecuting`] | The method is currently being executed by another thread. |
/// | [`MCManageError::NotReady`]           | The method can not be used.                               |
pub async fn check_allowed_start<T: ConcurrentClass>(class: &Arc<T>, restart: bool) -> Result<(), MCManageError> {
    match class.status().await {
        Status::Started => return Err(MCManageError::AlreadyExecuted),
        Status::Starting => return Err(MCManageError::CurrentlyExecuting),
        Status::Stopped => {
            class.set_status(Status::Starting).await;
            return Ok(())
        },
        Status::Stopping => return Err(MCManageError::NotReady),
        Status::Restarting => {
            if !restart {
                return Err(MCManageError::CurrentlyExecuting)
            } else {
                return Ok(())
            }
        }
    }
}

/// Check if the [`impl_stop`](ConcurrentClass::impl_stop) method is allowed to be executed. \
/// This function will also set the status of the given class to the right value. \
/// If the `forced` parameter got set to true this function will wait until the class has either started or stopped.
/// 
/// # Returns
/// 
/// | Return                                | Description                                               |
/// |---------------------------------------|-----------------------------------------------------------|
/// | [`Ok(())`]                            | The method can be executed immediately.                   |
/// | [`MCManageError::AlreadyExecuted`]    | The method has already been executed.                     |
/// | [`MCManageError::CurrentlyExecuting`] | The method is currently being executed by another thread. |
/// | [`MCManageError::NotReady`]           | The method can not be used.                               |
pub async fn check_allowed_stop<T: ConcurrentClass>(class: &Arc<T>, restart: bool, forced: bool) -> Result<(), MCManageError> {
    if forced && !restart {
        // wait till the class has started
        loop {
            let status = class.status().await;
            if status == Status::Started {
                break;
            }
            sleep(*class.config().refresh_rate()).await;
        }
    }
    
    match class.status().await {
        Status::Started => {
            class.set_status(Status::Stopping).await;
            return Ok(())
        }
        Status::Starting => return Err(MCManageError::NotReady),
        Status::Stopped => return Err(MCManageError::AlreadyExecuted),
        Status::Stopping => return Err(MCManageError::CurrentlyExecuting),
        Status::Restarting => {
            if !restart {
                return Err(MCManageError::NotReady)
            } else {
                return Ok(())
            }
        }
    }
}

/// Check if the [`impl_restart`](ConcurrentClass::impl_restart) method is allowed to be executed. \
/// This function will also set the status of the given class to the right value.
/// 
/// # Returns
/// 
/// | Return                                | Description                                                               |
/// |---------------------------------------|---------------------------------------------------------------------------|
/// | [`Ok(())`]                            | The method can be executed immediately.                                   |
/// | [`MCManageError::NotStarted`]         | The method can not be executed since the given struct is not yet started. |
/// | [`MCManageError::CurrentlyExecuting`] | The method is currently being executed by another thread.                 |
/// | [`MCManageError::NotReady`]           | The method can not be used.                                               |
pub async fn check_allowed_restart<T: ConcurrentClass>(class: &Arc<T>) -> Result<(), MCManageError> {
    match class.status().await {
        Status::Started => {
            class.set_status(Status::Restarting).await;
            return Ok(())
        }
        Status::Starting => return Err(MCManageError::NotReady),
        Status::Stopped => return Err(MCManageError::NotStarted),
        Status::Stopping => return Err(MCManageError::NotStarted),
        Status::Restarting => return Err(MCManageError::CurrentlyExecuting),
    }
}