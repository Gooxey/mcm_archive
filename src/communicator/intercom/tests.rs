#![allow(non_snake_case)]
#![cfg(test)]


use crate::config::Config;

use super::*;
use mcm_misc::message::message_type::MessageType;


// InterCom__add_handler tests
#[test]
fn InterCom__add_handler__valid_chars() {
    let (ic_tx, _rx) = mpsc::channel::<Message>();
    let (_, ic_rx) = mpsc::channel::<Message>();
    let myInterCom = InterCom::new(Arc::new(Config::new()), ic_tx, ic_rx);

    let (com_tx, com_rx) = mpsc::channel::<Message>();
    InterCom::set_communicator(&myInterCom, &Communicator::new(Arc::new(Config::new()), com_tx, com_rx));

    match InterCom::add_handler(&myInterCom, 'r') {
        Ok(r) => {
            let id = r.0;

            assert!(InterCom::get_lock_pure(&myInterCom, true).unwrap().handler_list.contains(&format!("{id}")), "The given id {} is missing in the handler_list.", format!("{id}"));
            assert!(InterCom::get_lock_pure(&myInterCom, true).unwrap().handlers.contains_key(&format!("{id}")), "The given key {} is missing in handlers.", format!("{id}"));
        }
        Err(e) => {
            assert!(false, "{}", e);
        }
    }
    match InterCom::add_handler(&myInterCom, 'c') {
        Ok(r) => {
            let id = r.0;

            assert!(InterCom::get_lock_pure(&myInterCom, true).unwrap().handler_list.contains(&format!("{id}")), "The given id {} is missing in the handler_list.", format!("{id}"));
            assert!(InterCom::get_lock_pure(&myInterCom, true).unwrap().handlers.contains_key(&format!("{id}")), "The given key {} is missing in handlers.", format!("{id}"));
        }
        Err(e) => {
            assert!(false, "{}", e);
        }
    }
}
#[test]
fn InterCom__add_handler__invalid_chars() {
    let (ic_tx, _) = mpsc::channel::<Message>();
    let (_, ic_rx) = mpsc::channel::<Message>();
    let myInterCom = InterCom::new(Arc::new(Config::new()), ic_tx, ic_rx);
    let (com_tx, com_rx) = mpsc::channel::<Message>();
    InterCom::set_communicator(&myInterCom, &Communicator::new(Arc::new(Config::new()), com_tx, com_rx));

    match InterCom::add_handler(&myInterCom, 'd') {
        Ok(r) => {
            let id = r.0;
            
            assert!(!InterCom::get_lock_pure(&myInterCom, true).unwrap().handler_list.contains(&format!("{id}")), "The invalid id {} was found in the handler_list.", format!("{id}"));
            assert!(!InterCom::get_lock_pure(&myInterCom, true).unwrap().handlers.contains_key(&format!("{id}")), "The invalid key {} was found in handlers.", format!("{id}"));
        }
        Err(e) => {
            match e {
                InterComError::InvalidType(_) => {
                    assert!(true)
                }
                _ => {
                    assert!(false, "{}", e)
                }
            }
        }
    }
    match InterCom::add_handler(&myInterCom, ' ') {
        Ok(r) => {
            let id = r.0;

            assert!(!InterCom::get_lock_pure(&myInterCom, true).unwrap().handler_list.contains(&format!("{id}")), "The invalid id {} was found in the handler_list.", format!("{id}"));
            assert!(!InterCom::get_lock_pure(&myInterCom, true).unwrap().handlers.contains_key(&format!("{id}")), "The invalid key {} was found in handlers.", format!("{id}"));
        }
        Err(e) => {
            match e {
                InterComError::InvalidType(_) => {
                    assert!(true)
                }
                _ => {
                    assert!(false, "{}", e)
                }
            }
        }
    }
}

// InterCom__remove_handler tests
#[test]
fn InterCom__remove_handler__existing_id() {
    let (ic_tx, _) = mpsc::channel::<Message>();
    let (_, ic_rx) = mpsc::channel::<Message>();
    let myInterCom = InterCom::new(Arc::new(Config::new()), ic_tx, ic_rx);
    let (com_tx, com_rx) = mpsc::channel::<Message>();
    InterCom::set_communicator(&myInterCom, &Communicator::new(Arc::new(Config::new()), com_tx, com_rx));
    

    let (id, _, _) = InterCom::add_handler(&myInterCom, 'r').unwrap();

    match InterCom::remove_handler(&myInterCom, &id.clone()) {
        Ok(_) => {
            assert!(!InterCom::get_lock_pure(&myInterCom, true).unwrap().handler_list.contains(&format!("{id}")), "The given id {} is still in the handler_list.", format!("{id}"));
            assert!(!InterCom::get_lock_pure(&myInterCom, true).unwrap().handlers.contains_key(&format!("{id}")), "The given key {} is still in handlers.", format!("{id}"));
        }
        Err(e) => {
            assert!(false, "{}", e);
        }
    }
}
#[test]
fn InterCom__remove_handler__nonexisting_id() {
    let (ic_tx, _) = mpsc::channel::<Message>();
    let (_, ic_rx) = mpsc::channel::<Message>();
    let myInterCom = InterCom::new(Arc::new(Config::new()), ic_tx, ic_rx);
    let (com_tx, com_rx) = mpsc::channel::<Message>();
    InterCom::set_communicator(&myInterCom, &Communicator::new(Arc::new(Config::new()), com_tx, com_rx));

    match InterCom::remove_handler(&myInterCom, &"r6".to_owned()) {
        Ok(_) => {
            assert!(false, "Expected the error: InterComError::IDNotFound.");
        }
        Err(e) => {
            match e {
                InterComError::IDNotFound(_) => {
                    assert!(true)
                }
                _ => {
                    assert!(false, "Expected the error: InterComError::IDNotFound. Found: {e}");
                }
            }
        }
    }
}
#[test]
fn InterCom__remove_handler__invalid_id() {
    let (ic_tx, _) = mpsc::channel::<Message>();
    let (_, ic_rx) = mpsc::channel::<Message>();
    let myInterCom = InterCom::new(Arc::new(Config::new()), ic_tx, ic_rx);
    let (com_tx, com_rx) = mpsc::channel::<Message>();
    InterCom::set_communicator(&myInterCom, &Communicator::new(Arc::new(Config::new()), com_tx, com_rx));

    match InterCom::remove_handler(&myInterCom, &"d0".to_owned()) {
        Ok(_) => {
            assert!(false, "Expected the error: InterComError::InvalidType.");
        }
        Err(e) => {
            match e {
                InterComError::InvalidType(_) => {
                    assert!(true)
                }
                _ => {
                    assert!(false, "Expected the error: InterComError::InvalidType. Found: {e}");
                }
            }
        }
    }
    match InterCom::remove_handler(&myInterCom, &" ".to_owned()) {
        Ok(_) => {
            assert!(false, "Expected the error: InterComError::InvalidType.");
        }
        Err(e) => {
            match e {
                InterComError::InvalidType(_) => {
                    assert!(true)
                }
                _ => {
                    assert!(false, "Expected the error: InterComError::InvalidType. Found: {e}");
                }
            }
        }
    }
}

// InterCom start/stop tests
#[test]
fn InterCom__start() {
    let (ic_tx, _receiver) = mpsc::channel::<Message>();
    let (_sender, ic_rx) = mpsc::channel::<Message>();
    let myInterCom = InterCom::new(Arc::new(Config::new()), ic_tx, ic_rx);
    let (com_tx, com_rx) = mpsc::channel::<Message>();
    InterCom::set_communicator(&myInterCom, &Communicator::new(Arc::new(Config::new()), com_tx, com_rx));

    InterCom::start(&myInterCom, true).unwrap();
    
    let myInterCom_lock = InterCom::get_lock_pure(&myInterCom, true).unwrap();

    // check if thread got created and alive status set to true
    match myInterCom_lock.main_thread {
        Some(_) => {
            assert!(true)
        }
        None => {
            assert!(false, "Expected thread to be created and saved to the `main_thread` field. Found nothing.")
        }
    }
    assert_eq!(myInterCom_lock.alive, true, "Expected `alive` field to be set to `true`.");

    drop(myInterCom_lock);

    InterCom::stop(&myInterCom, true).unwrap();
}
#[test]
fn InterCom__stop() {
    let (ic_tx, _receiver) = mpsc::channel::<Message>();
    let (_sender, ic_rx) = mpsc::channel::<Message>();
    let myInterCom = InterCom::new(Arc::new(Config::new()), ic_tx, ic_rx);
    let (com_tx, com_rx) = mpsc::channel::<Message>();
    InterCom::set_communicator(&myInterCom, &Communicator::new(Arc::new(Config::new()), com_tx, com_rx));

    InterCom::start(&myInterCom, true).unwrap();
    InterCom::stop(&myInterCom, true).unwrap();

    let myInterCom_lock = InterCom::get_lock_pure(&myInterCom, true).unwrap();

    // check if thread got deleted and alive status set to false
    match myInterCom_lock.main_thread {
        Some(_) => {
            assert!(false, "Expected thread to be deleted. Found a thread in the `main_thread` field.")
        }
        None => {
            assert!(true)
        }
    }
    assert_eq!(myInterCom_lock.alive, false, "Expected `alive` field to be set to `false`.");
}

// InterCom tests
#[test]
fn InterCom__Console_to_handler() {
    let (_sender, _) = mpsc::channel::<Message>();
    let (ic_tx, _receiver) = mpsc::channel::<Message>();
    let (tx, ic_rx) = mpsc::channel::<Message>();
    let myInterCom = InterCom::new(Arc::new(Config::new()), ic_tx, ic_rx);
    let (com_tx, com_rx) = mpsc::channel::<Message>();
    InterCom::set_communicator(&myInterCom, &Communicator::new(Arc::new(Config::new()), com_tx, com_rx));
    InterCom::start(&myInterCom, true).unwrap();

    let (id1, _placeholder1, rx1) = InterCom::add_handler(&myInterCom, 'c').unwrap();
    let (id2, _placeholder2, rx2) = InterCom::add_handler(&myInterCom, 'c').unwrap();

    // send messages

    tx.send(Message::new("test message to client 1", MessageType::Request,"", &id1, vec![])).unwrap();
    tx.send(Message::new("test message to client 2", MessageType::Request, "", &id2, vec![])).unwrap();
    
    
    // receive messages

    // client 1
    match rx1.try_recv() {
        Ok(data) => {
            assert_eq!(data.command(), &"test message to client 1".to_owned(), "Client 1 received the wrong message.")
        }
        Err(ref e) if *e == TryRecvError::Empty => { /* There is no message currently waiting to be received */ }
        Err(_) => {
            assert!(false, "The Message 1 did not get received by client 1");
        }
    }

    // client 2
    match rx2.try_recv() {
        Ok(data) => {
            assert_eq!(data.command(), &"test message to client 2".to_owned(), "Client 2 received the wrong message.")
        }
        Err(ref e) if *e == TryRecvError::Empty => { /* There is no message currently waiting to be received */ }
        Err(_) => {
            assert!(false, "The Message 2 did not get received by client 2");
        }
    }

    InterCom::stop(&myInterCom, true).unwrap();
}
#[test]
fn InterCom__handler_to_Console() {
    let (_placeholder, _receiver) = mpsc::channel::<Message>();
    let (ic_tx, rx) = mpsc::channel::<Message>();
    let (_sender, ic_rx) = mpsc::channel::<Message>();
    let myInterCom = InterCom::new(Arc::new(Config::new()), ic_tx, ic_rx);
    let (com_tx, com_rx) = mpsc::channel::<Message>();
    InterCom::set_communicator(&myInterCom, &Communicator::new(Arc::new(Config::new()), com_tx, com_rx));

    InterCom::start(&myInterCom, true).unwrap();

    let (id1, tx1, _placeholder2) = InterCom::add_handler(&myInterCom, 'c').unwrap();
    let (id2, tx2, _placeholder3) = InterCom::add_handler(&myInterCom, 'c').unwrap();

    // send messages
    tx1.send(Message::new("test message from client 1", MessageType::Request, &id1, "some_thread", vec![])).unwrap();
    tx2.send(Message::new("test message from client 2", MessageType::Request, "", &id2, vec![])).unwrap();

    // receive messages
    match rx.try_recv() {
        Ok(data) => {
            assert_eq!(data.command(), &"test message from client 1".to_owned(), "Console received the wrong message.")
        }
        Err(ref e) if *e == TryRecvError::Empty => { /* There is no message currently waiting to be received */ }
        Err(_) => {
            assert!(false, "The Message 1 did not get received by Console");
        }
    }        
    match rx.try_recv() {
        Ok(data) => {
            assert_eq!(data.command(), &"test message from client 2".to_owned(), "Console received the wrong message.")
        }
        Err(ref e) if *e == TryRecvError::Empty => { /* There is no message currently waiting to be received */ }
        Err(_) => {
            assert!(false, "The Message 2 did not get received by Console");
        }
    }

    InterCom::stop(&myInterCom, true).unwrap();
}