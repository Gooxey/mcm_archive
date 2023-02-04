// TEMP: will be removed when enough modules got finished
#![allow(dead_code)]

use std::sync::Arc;
use std::sync::mpsc;

use communicator::Communicator;
use config::Config;
use mcm_misc::message::Message;
use mcm_misc::config_trait::ConfigTrait;
use mcm_misc::concurrent_class::ConcurrentClass;

mod communicator;
mod config;
mod console;

fn main() {
    let config = Arc::new(Config::new());

    let (tx, rx) = mpsc::channel::<Message>();

    let com = Communicator::new(config.clone(), tx, rx);
    Communicator::start(&com, true).unwrap();
    Communicator::stop(&com, true).unwrap();
}