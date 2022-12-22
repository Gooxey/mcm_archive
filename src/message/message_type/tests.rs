#![allow(non_snake_case)]
#![cfg(test)]


use super::*;


#[test]
fn MessageType__from_str__valid() {
    if let Err(_) = MessageType::from_str("request") {
        assert!(false, "An error got returned when using the string `request`!")
    }
    if let Err(_) = MessageType::from_str("response") {
        assert!(false, "An error got returned when using the string `response`!")
    }
    if let Err(_) = MessageType::from_str("error") {
        assert!(false, "An error got returned when using the string `error`!")
    }
}
#[test]
fn MessageType__from_str__invalid() {
    if let Ok(_) = MessageType::from_str(" ") {
        assert!(false, "Expected an error by using an invalid string.")
    }
}

#[test]
fn MessageType__to_string() {
    assert_eq!(MessageType::Request.to_string(), "request".to_owned());
    assert_eq!(MessageType::Response.to_string(), "response".to_owned());
    assert_eq!(MessageType::Error.to_string(), "error".to_owned());
}