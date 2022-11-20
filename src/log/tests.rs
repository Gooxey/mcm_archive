#![allow(non_snake_case)]
#![cfg(test)]


use super::*;
  

#[test]
fn log__normal() {
    match log("info", "Proxy", "hello") {
        0 => {
            assert!(true);
        }
        1 => {
            assert!(true, "An invalid type error got returned.")
        }
        _ => {
            assert!(false, "The function is not supposed to return any numbers except 0 and 1.")
        }
    }
}
#[test]
fn log__invalid_type() {
    match log("invalid", "Proxy", "hello") {
        0 => {
            assert!(false, "Expected function to throw a invalid type error.");
        }
        1 => {
            assert!(true)
        }
        _ => {
            assert!(false, "The function is not supposed to return any numbers except 0 and 1.")
        }
    }
}
#[test]
fn log__space_text() {
    match log("info", " ", " ") {
        0 => {
            assert!(true);
        }
        1 => {
            assert!(true, "An invalid type error got returned.")
        }
        _ => {
            assert!(false, "The function is not supposed to return any numbers except 0 and 1.")
        }
    }
}