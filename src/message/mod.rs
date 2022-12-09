use serde_json::{Value, json};


mod tests;


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
    /// # fn main() {
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
    /// Create a new [`message`](Message) from a valid json.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter            | Description                                                |
    /// |----------------------|------------------------------------------------------------|
    /// | `json_object: Value` | The json object to create a new [`message`](Message) from. |
    /// 
    /// ## Example
    /// 
    /// ```
    /// use mcm_misc::message::Message;
    /// use serde_json::{Value, json};
    /// 
    /// # fn main() {
    /// let json: Value = json!({
    ///     "args": ["Hello world!"],
    ///     "command": "save_log",
    ///     "receiver": "proxy",
    ///     "sender":"r0"
    /// });
    /// let msg = Message::from_json(json);
    /// # }
    /// ```
    pub fn from_json(json_object: Value) -> Option<Self> {
        Some(Self {
            command: json_object["command"].as_str().unwrap().to_owned(),
            sender: json_object["sender"].as_str().unwrap().to_owned(),
            receiver: json_object["receiver"].as_str().unwrap().to_owned(),
            args: match json_object["args"].as_array() {
                Some(r) => {
                    let mut args = vec![];
                    for arg in r {
                        match arg.as_str() {
                            Some(rr) => { args.push(rr.to_owned()); }
                            None => { return None; }
                        }
                    }
                    args
                }
                None => { return None; }
            }
        })
    }
    /// Create a new [`message`](Message) from a valid string.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter        | Description                                           |
    /// |------------------|-------------------------------------------------------|
    /// | `string: String` | The string to create a new [`message`](Message) from. |
    /// 
    /// ## Example
    /// 
    /// ```
    /// use mcm_misc::message::Message;
    /// use serde_json::{Value, json};
    /// 
    /// # fn main() {
    /// let string: String = format!("{}", json!({"args":["Hello world!"],"command":"save_log","receiver":"proxy","sender":"r0"}));
    /// let msg = Message::from_string(string);
    /// # }
    /// ```
    pub fn from_string(string: String) -> Option<Self> {
        let json_object: Value = match serde_json::from_str(&string) {
            Ok(r) => { r }
            Err(_) => { return None; }
        };
        Self::from_json(json_object)
    }
    /// Create a new [`message`](Message) from a valid bytes string.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter               | Description                                                 |
    /// |-------------------------|------------------------------------------------------------ |
    /// | `bytes_string: Vec<u8>` | The bytes string to create a new [`message`](Message) from. |
    /// 
    /// ## Example
    /// 
    /// ```
    /// use mcm_misc::message::Message;
    /// use serde_json::{Value, json};
    /// 
    /// # fn main() {
    /// let bytes_string: Vec<u8> = format!("{}", json!({"args":["Hello world!"],"command":"save_log","receiver":"proxy","sender":"r0"})).as_bytes().to_owned();
    /// let msg = Message::from_bytes(bytes_string);
    /// # }
    /// ```
    pub fn from_bytes(bytes_string: Vec<u8>) -> Option<Self> {
        // strip the bytes_string from trailing characters
        let mut striped_bytes: Vec<u8> = vec![];
        for element in bytes_string {
            if element > 0 {
                striped_bytes.push(element);
            }
        }

        let json_object: Value = match serde_json::from_slice(&striped_bytes) {
            Ok(r) => { r }
            Err(_) => {
                return None;
            }
        };
        Self::from_json(json_object)
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