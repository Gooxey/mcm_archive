//! This module provides the [MCServerTypeError], which is used by the [MCServerType struct](super::MCServerType).


use thiserror::Error;


/// Errors used by the [`MCServerType struct`](super::MCServerType).
/// 
/// ## Variants
/// 
/// | Variant                                                               | Description                                                                                                    |
/// |-----------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------|
/// | [`ServerTypeNotFound(String)`](MCServerTypeError::ServerTypeNotFound) | The given server type could not be found. Check the `config/server_type.json` file for available server types. |
/// | [`NotAPlayerLine`](MCServerTypeError::NotAPlayerLine)                 | The given line does not contain a player's name.                                                               |
#[derive(Error, Debug)]
pub enum MCServerTypeError {
    /// The given server type could not be found. Check the `config/server_type.json` file for available server types.
    #[error("The server type {0} could not be found. Check the `config/server_type.json` file for available server types.")]
    ServerTypeNotFound(String),
    /// The given line does not contain a player's name.
    #[error("The given line does not contain a player's name.")]
    NotAPlayerLine
}