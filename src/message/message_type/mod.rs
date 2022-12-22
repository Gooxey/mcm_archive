mod tests;
mod msg_type_error;


use msg_type_error::MsgTypeError;


pub enum MessageType {
    Request,
    Response,
    Error
}
impl MessageType {
    pub fn from_str(string: &str) -> Result<Self, MsgTypeError> {
        match string {
            "request" => { return Ok(Self::Request) }
            "response" => { return Ok(Self::Response) }
            "error" => { return Ok(Self::Error) }
            _ => { Err(MsgTypeError::InvalidType(string.to_owned())) }
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            MessageType::Request => { return "request".to_owned() }
            MessageType::Response => { return "response".to_owned() }
            MessageType::Error => { return "error".to_owned() }
        }
    }
}