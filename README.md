# mcm_misc
Functions and structs used by applications in the MCManage Network.

## Description
This is part of the [MCManage](https://github.com/Gooxey/MCManage.git) project, which tries to make it as easy as possible to create and manage your [Minecraft](https://www.minecraft.net) servers.

### Features
- [Message struct](./src/message.rs) => A struct which holds all attributes of a standard message sent between applications in the MCManage Network
- [log function](./src/log.rs) => This function prints and/or saves a given string to the console or log file. A fancy mode will also be used if configured in the configuration of the application.

## Installation
Add the dependency to the `cargo.toml` file:
```
[dependencies]
mcm_misc = { path = "/path/to/mcm_misc/" }
```

Use the library:
```rust
use mcm_misc;

// code using the mcm_misc library
```
### Requirements
To use this library, rust and cargo have to be installed.

## Usage
```rust
use mcm_misc;

// using the Message struct
let msg = mcm_misc::Message::new("save_log", "r0", "proxy", vec!["hello world!"]);
let msg_bytes = msg.to_bytes();
// send the msg_bytes over a socket to the proxy


// using the log function
mcm_misc::log("info", "r0", "The log was sent to the proxy! ");
```


## License
[GNU General Public License v3.0](https://choosealicense.com/licenses/gpl-3.0/)
