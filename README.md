# mcm_misc
Functions and structs used by applications in the MCManage Network.

## Description
This is part of the [MCManage](https://github.com/Gooxey/MCManage.git) project, which tries to make it as easy as possible to create and manage your [Minecraft](https://www.minecraft.net) servers.

### Features
| Struct | Description |
|--------|-------------|
| [Message](./src/message.rs) | This struct represents the standard message, which is used to send commands or information between different applications in the MCManage network. |

| Function | Description |
|----------|-------------|
| [log](./src/log.rs) | This function prints and/or saves a given string to the console or log file. A fancy mode will also be used if configured in the configuration of the application. |


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
To use this library, [rust and cargo](https://www.rust-lang.org/tools/install) have to be installed.

## Usage
```rust
use mcm_misc;

// using the Message struct
let msg = mcm_misc::message::Message::new("save_log", "r0", "proxy", vec!["Hello world!"]);
let msg_bytes = msg.to_bytes();
// send the msg_bytes over a socket to the proxy


// using the log function
mcm_misc::log("info", "r0", "The log was sent to the proxy!");
```


## Roadmap
- Make the log function capable of automatically switching between fancy mode and normal mode based on the configuration.
- Make the log function capable of writing to a log file.
- SSwap integer returns with result returns.
- Make the config struct capable of setting its values based on a config file.


## License
[GNU General Public License v3.0](https://choosealicense.com/licenses/gpl-3.0/)
