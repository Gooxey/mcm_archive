use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MsgTypeError {
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