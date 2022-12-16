#![allow(non_snake_case)]
#![cfg(test)]


use super::*;


// InterCom__add_handler tests
#[test]
fn InterCom__add_handler__valid_chars() {
    let (ic_tx, _) = mpsc::channel::<Message>();
    let (_, ic_rx) = mpsc::channel::<Message>();
    let myInterCom = InterCom::new(Arc::new(Config::new()), ic_tx, ic_rx);

    match myInterCom.add_handler('r') {
        Ok(r) => {
            let id = r.0;

            assert!(myInterCom.handler_list.lock().unwrap().contains(&format!("{id}")), "The given id {} is missing in the handler_list.", format!("{id}"));
            assert!(myInterCom.handlers.lock().unwrap().contains_key(&format!("{id}")), "The given key {} is missing in handlers.", format!("{id}"));
        }
        Err(e) => {
            assert!(false, "{}", e);
        }
    }
    match myInterCom.add_handler('c') {
        Ok(r) => {
            let id = r.0;

            assert!(myInterCom.handler_list.lock().unwrap().contains(&format!("{id}")), "The given id {} is missing in the handler_list.", format!("{id}"));
            assert!(myInterCom.handlers.lock().unwrap().contains_key(&format!("{id}")), "The given key {} is missing in handlers.", format!("{id}"));
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

    match myInterCom.add_handler('d') {
        Ok(r) => {
            let id = r.0;
            
            assert!(!myInterCom.handler_list.lock().unwrap().contains(&format!("{id}")), "The invalid id {} was found in the handler_list.", format!("{id}"));
            assert!(!myInterCom.handlers.lock().unwrap().contains_key(&format!("{id}")), "The invalid key {} was found in handlers.", format!("{id}"));
        }
        Err(e) => {
            match e {
                ChannelError::InvalidType(_) => {
                    assert!(true)
                }
                _ => {
                    assert!(false, "{}", e)
                }
            }
        }
    }
    match myInterCom.add_handler(' ') {
        Ok(r) => {
            let id = r.0;

            assert!(!myInterCom.handler_list.lock().unwrap().contains(&format!("{id}")), "The invalid id {} was found in the handler_list.", format!("{id}"));
            assert!(!myInterCom.handlers.lock().unwrap().contains_key(&format!("{id}")), "The invalid key {} was found in handlers.", format!("{id}"));
        }
        Err(e) => {
            match e {
                ChannelError::InvalidType(_) => {
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
    let mut myInterCom = InterCom::new(Arc::new(Config::new()), ic_tx, ic_rx);

    let (id, _, _) = myInterCom.add_handler('r').unwrap();

    match myInterCom.remove_handler(id.clone()) {
        Ok(_) => {
            assert!(!myInterCom.handler_list.lock().unwrap().contains(&format!("{id}")), "The given id {} is still in the handler_list.", format!("{id}"));
            assert!(!myInterCom.handlers.lock().unwrap().contains_key(&format!("{id}")), "The given key {} is still in handlers.", format!("{id}"));
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
    let mut myInterCom = InterCom::new(Arc::new(Config::new()), ic_tx, ic_rx);

    match myInterCom.remove_handler("r6".to_owned()) {
        Ok(_) => {
            assert!(false, "Expected the error: ChannelError::IDNotFound.");
        }
        Err(e) => {
            match e {
                ChannelError::IDNotFound(_) => {
                    assert!(true)
                }
                _ => {
                    assert!(false, "Expected the error: ChannelError::IDNotFound. Found: {e}");
                }
            }
        }
    }
}
#[test]
fn InterCom__remove_handler__invalid_id() {
    let (ic_tx, _) = mpsc::channel::<Message>();
    let (_, ic_rx) = mpsc::channel::<Message>();
    let mut myInterCom = InterCom::new(Arc::new(Config::new()), ic_tx, ic_rx);

    match myInterCom.remove_handler("d0".to_owned()) {
        Ok(_) => {
            assert!(false, "Expected the error: ChannelError::InvalidType.");
        }
        Err(e) => {
            match e {
                ChannelError::InvalidType(_) => {
                    assert!(true)
                }
                _ => {
                    assert!(false, "Expected the error: ChannelError::InvalidType. Found: {e}");
                }
            }
        }
    }
    match myInterCom.remove_handler(" ".to_owned()) {
        Ok(_) => {
            assert!(false, "Expected the error: ChannelError::InvalidType.");
        }
        Err(e) => {
            match e {
                ChannelError::InvalidType(_) => {
                    assert!(true)
                }
                _ => {
                    assert!(false, "Expected the error: ChannelError::InvalidType. Found: {e}");
                }
            }
        }
    }
}

// InterCom start/stop tests
#[test]
fn InterCom__start() {
    let (ic_tx, receiver) = mpsc::channel::<Message>();
    let (sender, ic_rx) = mpsc::channel::<Message>();
    let mut myInterCom = InterCom::new(Arc::new(Config::new()), ic_tx, ic_rx);
    let myCommunicator = Communicator::start(Arc::new(Config::new()), sender, receiver).unwrap();

    myInterCom.start(&myCommunicator);

    // check if thread got created and alive status set to true
    match myInterCom.main_thread {
        Some(_) => {
            assert!(true)
        }
        None => {
            assert!(false, "Expected thread to be created and saved to the `main_thread` field. Found nothing.")
        }
    }
    assert_eq!(myInterCom.alive.load(Ordering::Relaxed), true, "Expected `alive` field to be set to `true`.");

    myInterCom.stop();
}
#[test]
fn InterCom__stop() {
    let (ic_tx, receiver) = mpsc::channel::<Message>();
    let (sender, ic_rx) = mpsc::channel::<Message>();
    let mut myInterCom = InterCom::new(Arc::new(Config::new()), ic_tx, ic_rx);
    let myCommunicator = Communicator::start(Arc::new(Config::new()), sender, receiver).unwrap();

    myInterCom.start(&myCommunicator);
    myInterCom.stop();

    // check if thread got deleted and alive status set to false
    match myInterCom.main_thread {
        Some(_) => {
            assert!(false, "Expected thread to be deleted. Found a thread in the `main_thread` field.")
        }
        None => {
            assert!(true)
        }
    }
    assert_eq!(myInterCom.alive.load(Ordering::Relaxed), false, "Expected `alive` field to be set to `false`.");
}

// InterCom tests
#[test]
fn InterCom__Console_to_handler() {
    let (sender, _) = mpsc::channel::<Message>();
    let (ic_tx, receiver) = mpsc::channel::<Message>();
    let (tx, ic_rx) = mpsc::channel::<Message>();
    let mut myInterCom: InterCom = InterCom::new(Arc::new(Config::new()), ic_tx, ic_rx);
    let myCommunicator = Communicator::start(Arc::new(Config::new()), sender, receiver).unwrap();

    myInterCom.start(&myCommunicator);

    let (id1, _placeholder1, rx1) = myInterCom.add_handler('c').unwrap();
    let (id2, _placeholder2, rx2) = myInterCom.add_handler('c').unwrap();

    // send messages

    tx.send(Message::new("test message to client 1", "", &id1, vec![])).unwrap();
    tx.send(Message::new("test message to client 2", "", &id2, vec![])).unwrap();
    
    
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

    myInterCom.stop();
}
#[test]
fn InterCom__handler_to_Console() {
    let (_placeholder, receiver) = mpsc::channel::<Message>();
    let (ic_tx, rx) = mpsc::channel::<Message>();
    let (sender, ic_rx) = mpsc::channel::<Message>();
    let mut myInterCom: InterCom = InterCom::new(Arc::new(Config::new()), ic_tx, ic_rx);
    let myCommunicator = Communicator::start(Arc::new(Config::new()), sender, receiver).unwrap();

    myInterCom.start(&myCommunicator);

    let (id1, tx1, _placeholder2) = myInterCom.add_handler('c').unwrap();
    let (id2, tx2, _placeholder3) = myInterCom.add_handler('c').unwrap();

    // send messages
    tx1.send(Message::new("test message from client 1", &id1, "some_thread", vec![])).unwrap();
    tx2.send(Message::new("test message from client 2", "", &id2, vec![])).unwrap();

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

    myInterCom.stop();
}