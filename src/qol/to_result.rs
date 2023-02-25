//! This module provides the [`to_result!`](crate::to_result) macro.

/// Convert Option to Result<T, [MCManageError](crate::mcmanage_error::MCManageError)>.
#[macro_export]
macro_rules! to_result {
    ($e: expr) => {
        match $e {
            Some(result) => Ok(result),
            None => Err($crate::mcmanage_error::MCManageError::UnwrapOnNone)
        }
    };
}