#![allow(non_snake_case)]
#![cfg(test)]


use std::{io::Write, fs};


use super::*;


#[test]
fn Command__to_string() {
    assert_eq!(Command::GetFile("ds".to_owned()).to_string(), "getfile")
}

#[test]
fn Command__getfile() {
    let filepath = "./test.txt".to_owned();
    
    // create a file to read
    let mut file = File::create(&filepath).unwrap();
    file.write("Hello world!".as_bytes()).unwrap();


    assert_eq!(Command::getfile(&filepath.to_owned()).unwrap(), "Hello world!".to_owned(), "The data read did not match the one written.");


    // remove the test file
    fs::remove_file(filepath).unwrap();
}