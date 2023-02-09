//! This module provides the [`MCServerManagerError`], which is used by the [`MCServerManager`](super::MCServerManager).


use std::io;

use thiserror::Error;

use crate::mcmanage_error::MCManageError;


/// Errors used by the [`MCServerManager struct`](super::MCServerManager).
/// 
/// ## Variants
/// 
/// | Variant                                               | Description                                                                                                                                                   |
/// |-------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------|
/// | [`InvalidFile`](MCServerManagerError::InvalidFile)    | The 'server_list.json' file was invalid. A valid example has been generated  under 'servers/server_list_example.json', and the invalid file has been renamed. |
/// | [`IOError(io::Error)`](MCServerManagerError::IOError) | An error occurred while opening the 'servers/server_list.json' file. A valid example will be generated under 'servers/server_list_example.json'.              |
/// | [`NotFound(`](MCServerManagerError::NotFound)         | The requested item could not be found.                                                                                                                        |
#[derive(Error, Debug)]
pub enum MCServerManagerError {
    /// The 'server_list.json' file was invalid. A valid example has been generated  under 'servers/server_list_example.json', and the invalid file has been renamed.
    #[error("The 'server_list.json' file was invalid. A valid example has been generated  under 'servers/server_list_example.json', and the invalid file has been renamed.")]
    InvalidFile,
    /// An error occurred while opening the 'servers/server_list.json' file. A valid example will be generated under 'servers/server_list_example.json'.
    #[error("An error occurred while opening the 'servers/server_list.json' file. A valid example will be generated under 'servers/server_list_example.json'. Error: {0}")]
    IOError(io::Error),
    /// The requested item could not be found.
    #[error("The requested item could not be found.")]
    NotFound,
    #[error(transparent)]
    MCManageError(#[from] MCManageError)
}