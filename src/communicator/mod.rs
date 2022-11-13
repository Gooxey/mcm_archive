//! This module represents the [`proxy's communicator`](Communicator). \
//! It accepts new connections from the MCManage network and manages the sending and receiving of [`messages`](mcm_misc::message::Message).

use self::intercom::InterCom;

mod intercom;

/// A placeholder for the real Communicator struct
struct Communicator {
    intercom: InterCom
}

/// A placeholder for the real handler method ( Most likely residing in the Communicator implementation )
fn handler() {

}