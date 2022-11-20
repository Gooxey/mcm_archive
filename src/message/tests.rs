#![allow(non_snake_case)]
#![cfg(test)]


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
        Some(new_msg) => {
            assert_eq!(new_msg["command"], json!("save_log"), "Expected the value `save_log` at the field `command`.");
            assert_eq!(new_msg["sender"], json!("r0"), "Expected the value `r0` at the field `sender`.");
            assert_eq!(new_msg["receiver"], json!("proxy"), "Expected the value `proxy` at the field `receiver`.");
            assert_eq!(new_msg["args"], json!(vec!["Hello world!"]), "Expected the value `vec!['Hello world!']` at the field `args`.");
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
        Some(new_msg) => {
            assert_eq!(new_msg["command"], json!(" "), "Expected the value ` ` at the field `command`.");
            assert_eq!(new_msg["sender"], json!(" "), "Expected the value ` ` at the field `sender`.");
            assert_eq!(new_msg["receiver"], json!(" "), "Expected the value ` ` at the field `receiver`.");
            assert_eq!(new_msg["args"], json!(vec![" "]), "Expected the value `vec![' ']` at the field `args`.");
        }
        None => {
            assert!(false, "Expected a json object.")
        }
    }
}
#[test]
fn Message__normal__to_string() {
    let msg = create_normal_message();

    match msg.to_string() {
        Some(new_msg) => {
            assert_eq!(new_msg, "{\"args\":[\"Hello world!\"],\"command\":\"save_log\",\"receiver\":\"proxy\",\"sender\":\"r0\"}", "The wrong string got returned.");
        }
        None => {
            assert!(false, "Expected a string.")
        }
    }
}
#[test]
fn Message__spaced__to_string() {
    let msg = create_spaced_message();

    match msg.to_string() {
        Some(new_msg) => {
            assert_eq!(new_msg, "{\"args\":[\" \"],\"command\":\" \",\"receiver\":\" \",\"sender\":\" \"}", "The wrong string got returned.");
        }
        None => {
            assert!(false, "Expected a string.")
        }
    }
}
#[test]
fn Message__normal__to_bytes() {
    let msg = create_normal_message();

    match msg.to_bytes() {
        Some(new_msg) => {
            let bytes_string = [123, 34, 97, 114, 103, 115, 34, 58, 91, 34, 72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33, 34, 93, 44, 34, 99, 111, 109, 109, 97, 110, 100, 34, 58, 34, 115, 97, 118, 101, 95, 108, 111, 103, 34, 44, 34, 114, 101, 99, 101, 105, 118, 101, 114, 34, 58, 34, 112, 114, 111, 120, 121, 34, 44, 34, 115, 101, 110, 100, 101, 114, 34, 58, 34, 114, 48, 34, 125];
            assert_eq!(new_msg, bytes_string, "The wrong bytes string got returned.");
        }
        None => {
            assert!(false, "Expected a bytes string.")
        }
    }
}
#[test]
fn Message__spaced__to_bytes() {
    let msg = create_spaced_message();

    match msg.to_bytes() {
        Some(new_msg) => {
            let bytes_string = [123, 34, 97, 114, 103, 115, 34, 58, 91, 34, 32, 34, 93, 44, 34, 99, 111, 109, 109, 97, 110, 100, 34, 58, 34, 32, 34, 44, 34, 114, 101, 99, 101, 105, 118, 101, 114, 34, 58, 34, 32, 34, 44, 34, 115, 101, 110, 100, 101, 114, 34, 58, 34, 32, 34, 125];
            assert_eq!(new_msg, bytes_string, "The wrong bytes string got returned.");
        }
        None => {
            assert!(false, "Expected a bytes string.")
        }
    }
}

#[test]
fn Message__normal__from_json() {
    let msg = create_normal_message();
    let json_msg = msg.to_json().unwrap();

    match Message::from_json(json_msg.clone()) {
        Some(new_msg) => {
            match new_msg.to_json() {
                Some(m) => {
                    assert_eq!(m, json_msg, "The json object of the original message did not equal the json object of the message received by the `from_json` method.");
                }
                None => {
                    assert!(false, "The message received by the `from_json` method could not be converted back to a JSON object.")
                }
            }
        }
        None => {
            assert!(false, "The `from_json` method did not return anything.")
        }
    }
}
#[test]
fn Message__spaced__from_json() {
    let msg = create_spaced_message();
    let json_msg = msg.to_json().unwrap();

    match Message::from_json(json_msg.clone()) {
        Some(new_msg) => {
            match new_msg.to_json() {
                Some(m) => {
                    assert_eq!(m, json_msg, "The json object of the original message did not equal the json object of the message received by the `from_json` method.");
                }
                None => {
                    assert!(false, "The message received by the `from_json` method could not be converted back to a JSON object.")
                }
            }
        }
        None => {
            assert!(false, "The `from_json` method did not return anything.")
        }
    }
}
#[test]
fn Message__normal__from_string() {
    let msg = create_normal_message();
    let str_msg = msg.to_string().unwrap();

    match Message::from_string(str_msg.clone()) {
        Some(new_msg) => {
            match new_msg.to_string() {
                Some(m) => {
                    assert_eq!(m, str_msg, "The string of the original message did not equal the string of the message received by the `from_string` method.");
                }
                None => {
                    assert!(false, "The message received by the `from_string` method could not be converted back to a string.")
                }
            }
        }
        None => {
            assert!(false, "The `from_string` method did not return anything.")
        }
    }
}
#[test]
fn Message__spaced__from_string() {
    let msg = create_spaced_message();
    let str_msg = msg.to_string().unwrap();

    match Message::from_string(str_msg.clone()) {
        Some(new_msg) => {
            match new_msg.to_string() {
                Some(m) => {
                    assert_eq!(m, str_msg, "The string of the original message did not equal the string of the message received by the `from_string` method.");
                }
                None => {
                    assert!(false, "The message received by the `from_string` method could not be converted back to a string.")
                }
            }
        }
        None => {
            assert!(false, "The `from_string` method did not return anything.")
        }
    }
}
#[test]
fn Message__normal__from_bytes() {
    let msg = create_normal_message();
    let bytes_msg = msg.to_bytes().unwrap();

    match Message::from_bytes(bytes_msg.clone()) {
        Some(new_msg) => {
            match new_msg.to_bytes() {
                Some(m) => {
                    assert_eq!(m, bytes_msg, "The bytes string of the original message did not equal the bytes string of the message received by the `from_bytes` method.");
                }
                None => {
                    assert!(false, "The message received by the `from_bytes` method could not be converted back to a bytes string.")
                }
            }
        }
        None => {
            assert!(false, "The `from_bytes` method did not return anything.")
        }
    }
}
#[test]
fn Message__spaced__from_bytes() {
    let msg = create_spaced_message();
    let bytes_msg = msg.to_bytes().unwrap();

    match Message::from_bytes(bytes_msg.clone()) {
        Some(new_msg) => {
            match new_msg.to_bytes() {
                Some(m) => {
                    assert_eq!(m, bytes_msg, "The bytes string of the original message did not equal the bytes string of the message received by the `from_bytes` method.");
                }
                None => {
                    assert!(false, "The message received by the `from_bytes` method could not be converted back to a bytes string.")
                }
            }
        }
        None => {
            assert!(false, "The `from_bytes` method did not return anything.")
        }
    }
}
