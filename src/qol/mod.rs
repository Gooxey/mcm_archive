//! This module provides some quality-of-life macros. 
//! 
//! # Macros
//! 
//! | Macro                          | Description                                                                         |
//! |--------------------------------|-------------------------------------------------------------------------------------|
//! | [log_print!](crate::log_print) | This macro can be used to print a given string to the console.                      |
//! | [log!](crate::log)             | This macro can be used to print and save a given string to a file or the console.   |
//! | [to_result!](crate::to_result) | Convert Option to Result<T, [MCManageError](crate::mcmanage_error::MCManageError)>. |

pub mod log;
pub mod to_result;