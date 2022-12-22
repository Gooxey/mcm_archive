mod tests;
pub mod msg_type_error;


use msg_type_error::MsgTypeError;


/// This enum describes the type of message holding this enum. There are three possible states. The first one is [`request`](MessageType::Request). It is used when the
/// [`Message`](super::Message) was sent to receive or execute something. The second option, [`response`](MessageType::Response), is used when the [`Message`](super::Message)
/// returns the requested data or the result of an operation. The last variant gets used in case an error occurs while executing a command.
/// 
/// ## Methods
/// 
/// | Method                                                  | Description                                                           |
/// |---------------------------------------------------------|-----------------------------------------------------------------------|
/// | [`from_str(...) -> Result<...>`](MessageType::from_str) | Create this enum based on a string provided.                          |
/// | [`to_string(...) -> String`](MessageType::to_string)    | Convert the [`message's`](Message) data into a json_object.           |
pub enum MessageType {
    Request,
    Response,
    Error
}
impl MessageType {
    /// Create this enum based on the string provided. An error will be returned if the provided string is invalid.
    pub fn from_str(string: &str) -> Result<Self, MsgTypeError> {
        match string {
            "request" => { return Ok(Self::Request) }
            "response" => { return Ok(Self::Response) }
            "error" => { return Ok(Self::Error) }
            _ => { Err(MsgTypeError::InvalidType(string.to_owned())) }
        }
    }
    /// Return a string that can be used to create the same kind of MessageType.
    pub fn to_string(&self) -> String {
        match self {
            MessageType::Request => { return "request".to_owned() }
            MessageType::Response => { return "response".to_owned() }
            MessageType::Error => { return "error".to_owned() }
        }
    }
}