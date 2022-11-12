use serde_json::{Value, json};

pub struct Message {
    command: String,
    sender: String,
    receiver: String,
    args: Vec<String>
}
impl Message {
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

    /// Convert the Messages data into a json_object.
    pub fn to_json(&self) -> Option<Value> {
        Some(json!({
            "command": self.command,
            "sender": self.sender,
            "receiver": self.receiver,
            "args": self.args
        }))
    }
    /// Convert the Messages data into a string.
    pub fn to_string(&self) -> Option<String> {
        match Self::to_json(&self) {
            Some(json_object) => {
                Some(format!("{json_object}"))
            }
            None => None
        }
    }
    /// Convert the Messages data into an owned bytes-string.
    pub fn to_bytes(&self) -> Option<Vec<u8>> {
        match  Self::to_string(&self) {
            Some(str) => Some(str.as_bytes().to_owned()),
            None => None
        }
    }

    // Getter methods

    pub fn command(&self) -> &String {
        &self.command
    }
    pub fn sender(&self) -> &String {
        &self.sender
    }
    pub fn receiver(&self) -> &String {
        &self.receiver
    }
    pub fn args(&self) -> &Vec<String> {
        &self.args
    }
}