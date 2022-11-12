use serde_json::{Value, json};

/// This struct represents the standard message, which is used to send commands or information between different applications in the MCManage network. \
/// It also has methods to convert the given data to a JSON object, string, or bytes object.
/// 
/// ## Methods
/// 
/// | Method                                                | Description                                                       |
/// |-------------------------------------------------------|-------------------------------------------------------------------|
/// | [`new(...) -> Self`](Message::new)                    | Create a new [`message`](Message).                                |
/// | [`to_json(...) -> Option<...>`](Message::to_json)     | Convert the [`message's`](Message) data into a json_object.       |
/// | [`to_string(...) -> Option<...>`](Message::to_string) | Convert the [`message's`](Message) data into a string.            |
/// | [`to_bytes(...) -> Option<...>`](Message::to_bytes)   | Convert the [`message's`](Message) data into a bytes-string.      |
/// |                                                       |                                                                   |
/// | [`command() -> &String`](Message::command)            | Returns a reference to the [`message's`](Message) command field.  |
/// | [`sender() -> &String`](Message::sender)              | Returns a reference to the [`message's`](Message) sender field.   |
/// | [`receiver() -> &String`](Message::receiver)          | Returns a reference to the [`message's`](Message) receiver field. |
/// | [`args() -> &Vec<String>`](Message::args)             | Returns a reference to the [`message's`](Message) args field.     |
pub struct Message {
    /// The command to send.
    command: String,
    /// The ID of the application sending this message.
    sender: String,
    /// The ID of the application the message is meant for.
    receiver: String,
    /// Any additional information.
    args: Vec<String>
}
impl Message {
    /// Create a new [`message`](Message).
    /// 
    /// ## Parameters
    /// 
    /// | Parameter         | Description                                                      |
    /// |-------------------|------------------------------------------------------------------|
    /// | `command: &str`   | The command to send.                                             |
    /// | `sender: &str`    | The ID of the application sending this [`message`](Message).     |
    /// | `receiver: &str`  | The ID of the application the [`message`](Message) is meant for. |
    /// | `args: Vec<&str>` | Any additional information.                                      |
    /// 
    /// ## Example
    /// 
    /// ```
    /// use mcm_misc::message::Message;
    /// 
    /// # fn main() {    /// 
    /// let msg = Message::new("save_log", "r0", "proxy", vec!["hello world!"]);
    /// # }
    /// ```
    pub fn new(command: &str, sender: &str, receiver: &str, args: Vec<&str>) -> Self {
        Self {
            command: command.to_owned(),
            sender: sender.to_owned(),
            receiver: receiver.to_owned(),
            args: Self::vec_items_to_owned(args)
        }
    }

    /// Convert the vectors items to owned ones. \
    /// This will consume the given vector.
    fn vec_items_to_owned(vector: Vec<&str>) -> Vec<String>{
        let mut new_vector: Vec<String> = vec![];
        
        for item in vector {
            new_vector.push(item.to_owned());
        }
        new_vector
    }

    /// Convert the [`message's`](Message) data into a json_object. \
    /// The result will be returned.
    pub fn to_json(&self) -> Option<Value> {
        Some(json!({
            "command": self.command,
            "sender": self.sender,
            "receiver": self.receiver,
            "args": self.args
        }))
    }
    /// Convert the [`message's`](Message) data into a string. \
    /// The result will be returned.
    pub fn to_string(&self) -> Option<String> {
        match Self::to_json(&self) {
            Some(json_object) => {
                Some(format!("{json_object}"))
            }
            None => None
        }
    }
    /// Convert the [`message's`](Message) data into a bytes-string. \
    /// The result will be returned.
    pub fn to_bytes(&self) -> Option<Vec<u8>> {
        match  Self::to_string(&self) {
            Some(str) => Some(str.as_bytes().to_owned()),
            None => None
        }
    }

    /// Returns a reference to the [`message's`](Message) command field.
    pub fn command(&self) -> &String {
        &self.command
    }
    /// Returns a reference to the [`message's`](Message) sender field.
    pub fn sender(&self) -> &String {
        &self.sender
    }
    /// Returns a reference to the [`message's`](Message) receiver field.
    pub fn receiver(&self) -> &String {
        &self.receiver
    }
    /// Returns a reference to the [`message's`](Message) args field.
    pub fn args(&self) -> &Vec<String> {
        &self.args
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;
    
    /// Make a new message using text as the input strings. 
    fn create_normal_message() -> Message {
        Message::new("save_log", "r0", "proxy", vec!["Hello world!"])
    }
    /// Make a new message using spaces as the input strings. 
    fn create_spaced_message() -> Message {
        Message::new(" ", " ", " ", vec![" "])
    }

    #[test]
    fn Message__normal__to_json() {
        let msg = create_normal_message();

        match msg.to_json() {
            Some(json_msg) => {
                if json_msg["command"] != json!("save_log") {
                    assert!(false, "Expected the value `save_log` at the field `command`.")
                }
                if json_msg["sender"] != json!("r0") {
                    assert!(false, "Expected the value `r0` at the field `sender`.")
                }
                if json_msg["receiver"] != json!("proxy") {
                    assert!(false, "Expected the value `proxy` at the field `receiver`.")
                }
                if json_msg["args"] != json!(vec!["Hello world!"]) {
                    assert!(false, "Expected the value `vec!['Hello world!']` at the field `args`.")
                }
                assert!(true);
            }
            None => {
                assert!(false, "Expected a json object.")
            }
        }
    }
    #[test]
    fn Message__spaced__to_json() {
        let msg = create_spaced_message();

        match msg.to_json() {
            Some(json_msg) => {
                if json_msg["command"] != json!(" ") {
                    assert!(false, "Expected the value ` ` at the field `command`.")
                }
                if json_msg["sender"] != json!(" ") {
                    assert!(false, "Expected the value ` ` at the field `sender`.")
                }
                if json_msg["receiver"] != json!(" ") {
                    assert!(false, "Expected the value ` ` at the field `receiver`.")
                }
                if json_msg["args"] != json!(vec![" "]) {
                    assert!(false, "Expected the value `vec![' ']` at the field `args`.")
                }
                assert!(true);
            }
            None => {
                assert!(false, "Expected a json object.")
            }
        }
    }
    #[test]
    fn Message__normal__to_string() {
        let msg = create_normal_message();

        match msg.to_json() {
            Some(json_msg) => {
                if json_msg["command"] != json!("save_log") {
                    assert!(false, "Expected the value `save_log` at the field `command`.")
                }
                if json_msg["sender"] != json!("r0") {
                    assert!(false, "Expected the value `r0` at the field `sender`.")
                }
                if json_msg["receiver"] != json!("proxy") {
                    assert!(false, "Expected the value `proxy` at the field `receiver`.")
                }
                if json_msg["args"] != json!(vec!["Hello world!"]) {
                    assert!(false, "Expected the value `vec!['Hello world!']` at the field `args`.")
                }
                assert!(true);
            }
            None => {
                assert!(false, "Expected a json object.")
            }
        }
    }
    #[test]
    fn Message__spaced__to_string() {
        let msg = create_spaced_message();

        match msg.to_json() {
            Some(json_msg) => {
                if json_msg["command"] != json!(" ") {
                    assert!(false, "Expected the value ` ` at the field `command`.")
                }
                if json_msg["sender"] != json!(" ") {
                    assert!(false, "Expected the value ` ` at the field `sender`.")
                }
                if json_msg["receiver"] != json!(" ") {
                    assert!(false, "Expected the value ` ` at the field `receiver`.")
                }
                if json_msg["args"] != json!(vec![" "]) {
                    assert!(false, "Expected the value `vec![' ']` at the field `args`.")
                }
                assert!(true);
            }
            None => {
                assert!(false, "Expected a json object.")
            }
        }
    }
    #[test]
    fn Message__normal__to_bytes() {
        let msg = create_normal_message();

        match msg.to_json() {
            Some(json_msg) => {
                if json_msg["command"] != json!("save_log") {
                    assert!(false, "Expected the value `save_log` at the field `command`.")
                }
                if json_msg["sender"] != json!("r0") {
                    assert!(false, "Expected the value `r0` at the field `sender`.")
                }
                if json_msg["receiver"] != json!("proxy") {
                    assert!(false, "Expected the value `proxy` at the field `receiver`.")
                }
                if json_msg["args"] != json!(vec!["Hello world!"]) {
                    assert!(false, "Expected the value `vec!['Hello world!']` at the field `args`.")
                }
                assert!(true);
            }
            None => {
                assert!(false, "Expected a json object.")
            }
        }
    }
    #[test]
    fn Message__spaced__to_bytes() {
        let msg = create_spaced_message();

        match msg.to_json() {
            Some(json_msg) => {
                if json_msg["command"] != json!(" ") {
                    assert!(false, "Expected the value ` ` at the field `command`.")
                }
                if json_msg["sender"] != json!(" ") {
                    assert!(false, "Expected the value ` ` at the field `sender`.")
                }
                if json_msg["receiver"] != json!(" ") {
                    assert!(false, "Expected the value ` ` at the field `receiver`.")
                }
                if json_msg["args"] != json!(vec![" "]) {
                    assert!(false, "Expected the value `vec![' ']` at the field `args`.")
                }
                assert!(true);
            }
            None => {
                assert!(false, "Expected a json object.")
            }
        }
    }
}