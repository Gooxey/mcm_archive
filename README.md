# mcm_misc
Functions and structs used by applications in the [MCManage Network](https://github.com/Gooxey/MCManage.git).

## Description
This is part of the [MCManage](https://github.com/Gooxey/MCManage.git) project, which tries to make it as easy as possible to create and manage your [Minecraft servers](https://www.minecraft.net).

### Features
| Struct | Description |
|--------|-------------|
| [Message](./src/message/mod.rs) | This struct represents the standard message, which is used to send commands or information between different applications in the MCManage network. |

| Enum | Description |
|------|-------------|
| [MessageType](./src/message/message_type/mod.rs) | This enum describes the type of message holding this enum. |

| Function | Description |
|----------|-------------|
| [log](./src/log/mod.rs) | This function prints and/or saves a given string to the console or log file. A fancy mode will also be used if configured in the configuration of the application. |

| Trait | Description |
|-------|-------------|
| [Config](./src/config.rs) | Every struct implementing this trait can be used as the application's config. |


## Installation
Add the dependency to the `cargo.toml` file:
```
[dependencies]
mcm_misc = { git = "https://github.com/Gooxey/mcm_misc.git", version = "X.Y.Z" }
    or
mcm_misc = { path = "/path/to/mcm_misc/" }
```

Use the library:
```rust
use mcm_misc;

// code using the mcm_misc library
```

### Requirements
To use this library, [rust and cargo](https://www.rust-lang.org/tools/install) have to be installed.

## License
[GNU General Public License v3.0](https://choosealicense.com/licenses/gpl-3.0/)
