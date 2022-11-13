//! This module represents the [`proxy's communicator`](Communicator). \
//! It accepts new connections from the MCManage network and manages the sending and receiving of [`messages`](mcm_misc::message::Message).\
//! \
//! ```text
//! -------------------------------------
//! |           Communicator            |
//! |                                   |
//! |    ----------------------------   |
//! |    | Communicator main thread |   |   Creates a handler for each new connection.
//! |    ----------------------------   |
//! |         |                |        |
//! |         |                |        |
//! |         V                V        |
//! |    -----------      -----------   |   Send and receive messages to and from their 
//! |    | Handler |      | Handler |   |   connected client.
//! |    -----------      -----------   |   A client could be a Runner or a Client application.
//! |      Λ                      Λ     |
//! |      |                      |     |   Send and receive channels to transmit messages.
//! |      |                      |     |
//! |      |     ------------     |     |   Passes on received messages to the right receiver.
//! |      ----> | InterCom | <----     |   This can be a handler or the console.
//! |            ------------           |   
//! |                 Λ                 |
//! |                 |                 |
//! ----------------- | -----------------   Send and receive channels to transmit messages.
//!                   |
//!                   V
//! -------------------------------------   Here, all received commands get executed and
//! |              Console              |   messages for clients are created.
//! -------------------------------------
//! ```

use self::intercom::InterCom;

mod intercom;

/// A placeholder for the real Communicator struct
struct Communicator {
    intercom: InterCom
}

/// A placeholder for the real handler method ( Most likely residing in the Communicator implementation )
fn handler() {

}