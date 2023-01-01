//! This module provides an error type used by the [`MessageType enum`](super::MessageType).


use std::error::Error;
use std::fmt;


/// This error type gets used by the [`MessageType enum`](super::MessageType).
/// 
/// ## Variants
/// 
/// | Variant                                            | Description                                                                                         |
/// |----------------------------------------------------|-----------------------------------------------------------------------------------------------------|
/// | [`InvalidType(String)`](MsgTypeError::InvalidType) | The type provided is not supported. Please use one of the following: `request`, `response`, `error` |
#[derive(Debug)]
pub enum MsgTypeError {
    /// The type provided is not supported. Please use one of the following: `request`, `response`, `error`
    InvalidType(String)
}
impl fmt::Display for MsgTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MsgTypeError::InvalidType(invalid_type) => {
                write!(f, "The type `{invalid_type}` is not supported. Please use one of the following: `request`, `response`, `error`")
            }
            
        }
    }
}
impl Error for MsgTypeError {}