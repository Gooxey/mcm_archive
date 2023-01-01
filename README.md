# mcm_misc
Functions and structs used by applications in the [MCManage Network](https://github.com/Gooxey/MCManage.git).

## Description
This is part of the [MCManage](https://github.com/Gooxey/MCManage.git) project, which tries to make it as easy as possible to create and manage your [Minecraft servers](https://www.minecraft.net).

### Features
| Struct | Description |
|--------|-------------|
| [Message](./src/message/mod.rs) | This struct represents the standard message, which is used to send commands or information between different applications in the [`MCManage network`](https://github.com/Gooxey/MCManage.git). |

| Enum | Description |
|------|-------------|
| [MessageType](./src/message/message_type/mod.rs) | This enum describes the type of message holding this enum. |

| Trait | Description |
|-------|-------------|
| [Config](./src/config.rs) | Every struct implementing this trait can be used as the application's config. |

| Error | Description |
|-------|-------------|
| [MsgTypeError](./src/message/message_type/msg_type_error.rs) | This error type gets used by the [`MessageType enum`](./src/message/message_type/mod.rs). |

| Function | Description |
|----------|-------------|
| [log](./src/log/mod.rs) | This function can be used to print and save a given string to a file or the console. This can be done in a fancy mode (colored text) if enabled by the [`application's config`](./src/config.rs) |

## Installation
Add the dependency to the `cargo.toml` file:
```
[dependencies]
mcm_misc = { git = "https://github.com/Gooxey/mcm_misc.git", version = "X.Y.Z" }
    or
mcm_misc = { path = "/path/to/mcm_misc/" }
```

## Requirements
To use this library, [rust and cargo](https://www.rust-lang.org/tools/install) have to be installed.

## License
[GNU General Public License v3.0](https://choosealicense.com/licenses/gpl-3.0/)
