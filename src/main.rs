// TEMP: will be removed when enough modules got finished
#![allow(dead_code)]

use std::sync::Arc;
use std::sync::mpsc;

use communicator::Communicator;
use config::Config;
use mcm_misc::message::Message;

mod communicator;
mod config;
mod error;
mod console;

fn main() {
    let config = Arc::new(Config::new());

    let (tx, rx) = mpsc::channel::<Message>();

    let com = communicator::Communicator::start(config.clone(), tx, rx).unwrap();
    Communicator::stop(&com);
}