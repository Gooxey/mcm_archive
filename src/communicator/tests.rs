#![allow(non_snake_case)]
#![cfg(test)]


use std::sync::mpsc;

use super::*;


fn communicator_init_values() -> (Arc<Config>, Sender<Message>, Receiver<Message>, Sender<Message>, Receiver<Message>) {
    let (tx, rx_com) = mpsc::channel::<Message>();
    let (tx_com, rx) = mpsc::channel::<Message>();

    (Arc::new(Config::new()), tx_com, rx_com, tx, rx)
}
fn new_Message() -> Message {
    Message::new("save_log", MessageType::Request, "proxy", "r0", vec!["hello world!"])
}
fn register_client(client: &mut TcpStream) {
    let mut buffer = [0; 110];

    match client.read(&mut buffer) {
        Ok(n) => {
            if n <= buffer.len() {
                if Message::from_bytes(buffer.to_vec()).unwrap().command() == "get_client_type" {
                    client.write(&Message::new("get_client_type", MessageType::Response, "", "", vec!["r"]).to_bytes().unwrap()).unwrap();
                    
                    // wait for a confirmation that this connection got registered as a runner connection
                    match client.read(&mut buffer) {
                        Ok(_) => {}
                        Err(e) => { assert!(false, "An error occurred while receiving the confirm from the Communicator. Error: {e}")}
                    }
                    
                } else {
                    assert!(false, "The command `get_client_type` was not received");
                }
            }
        }
        Err(e) => {
            assert!(false, "An error occurred while reading a message from the client. Error: {e}");
        }
    }
}

#[test]
fn Communicator__start() {
    let (config, sender, receiver, _tx, _rx) = communicator_init_values();
    let com = Communicator::start(config, sender, receiver).unwrap();

    if let Ok(com) = com.lock() {
        assert_eq!(com.alive.load(Ordering::Relaxed), true, "Expected `alive` field to be true.");
        if let None = com.main_thread  {
            assert!(false, "Expected `main_thread` field to not be None.");
        }
    };
    Communicator::stop(&com);
}
#[test]
fn Communicator__stop() {
    let (config, sender, receiver, _tx, _rx) = communicator_init_values();
    let com = Communicator::start(config, sender, receiver).unwrap();
    
    Communicator::stop(&com);
    if let Ok(com) = com.lock() {
        assert_eq!(com.alive.load(Ordering::Relaxed), false, "Expected `alive` field to be false.");
        if let Some(_) = com.main_thread  {
            assert!(false, "Expected `main_thread` field to be None.");
        }
    };
}
#[test]
fn Communicator__restart() {    
    let (config, sender, receiver, _tx, _rx) = communicator_init_values();
    let com = Communicator::start(config, sender, receiver).unwrap();
    
    Communicator::restart(&com, None, None).unwrap();

    if let Ok(com) = com.lock() {
        assert_eq!(com.alive.load(Ordering::Relaxed), true, "Expected `alive` field to be true.");
        if let None = com.main_thread  {
            assert!(false, "Expected `main_thread` field to be Some.");
        }
    };
    Communicator::stop(&com);
}

#[test]
fn Communicator__InterCom_to_Client() {
    let (config, sender, receiver, tx, _rx) = communicator_init_values();
    let com = Communicator::start(config, sender, receiver).unwrap();

    let mut client = TcpStream::connect(Config::new().addr()).unwrap();
    let mut buffer = [0; 110];

    register_client(&mut client);

    if let Err(_) = client.set_nonblocking(true) {
        assert!(false, "Failed to activate the `nonblocking` mode for a client.");
    }

    if let Err(err) = tx.send(new_Message()) {
        assert!(false, "An error occurred while sending a Message to the Communicator. Error: {err}");
    };


    loop {
        match client.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    assert!(false, "The Communicator disconnected.");
                } else {
                    assert_eq!(new_Message().to_string(), Message::from_bytes(buffer.to_vec()).unwrap().to_string(), "The received message did not equal the one sent.");
                }
                break;
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => { /* no message waiting to be read */ }
            Err(e) => {
                assert!(false, "An error occurred while reading a message from the client. Error: {e}");
            }
        }
    }
    Communicator::stop(&com);
}
#[test]
fn Communicator__Client_to_InterCom() {
    let (config, sender, receiver, _tx, rx) = communicator_init_values();
    let com = Communicator::start(config, sender, receiver).unwrap();
    let mut client = TcpStream::connect(Config::new().addr()).unwrap();

    let message = Message::new("save_log", MessageType::Request, "r0", "some_thread", vec!["hello world!"]);
    
    register_client(&mut client);

    if let Err(err) = client.write(&message.to_bytes().unwrap()) {
        assert!(false, "An error occurred while sending a Message to the Communicator. Error: {err}");
    }

    loop {
        match rx.try_recv() {
            Ok(msg) => {
                assert_eq!(message.to_string(), msg.to_string(), "The received message did not equal the one sent.");
                break;
            }
            Err(e) if e == TryRecvError::Empty => { /* no message currently waiting to be received */ }
            Err(_) => {
                assert!(false, "The connection to the Communicator got interrupted.");
            }
        }
    }

    Communicator::stop(&com);
}