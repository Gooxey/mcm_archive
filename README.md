# mcm_misc

Functions and structs used by applications in the [MCManage Network](https://github.com/Gooxey/MCManage.git).

## Description

This is part of the [MCManage](https://github.com/Gooxey/MCManage.git) project, which tries to make it as easy as possible to create and manage your [Minecraft servers](https://www.minecraft.net).

### General

| Error | Description |
|-|-|
| [MCManageError](./src/mcmanage_error.rs) | This error type provides errors used almost anywhere in the [MCManage network](https://github.com/Gooxey/MCManage.git). |

| Macros | Description |
|-|-|
| [log!](./src/log.rs)       | This macro can be used to print a given string to the console.                    |
| [log_print!](./src/log.rs) | This macro can be used to print and save a given string to a file or the console. |

| Trait | Description |
|-|-|
| [ConcurrentClass](./src/concurrent_class.rs) | This trait provides standard functions used by every concurrent struct in the [MCManage network](https://github.com/Gooxey/MCManage.git). |

### Message

| Struct | Description |
|-|-|
| [Message](./src/message/mod.rs) | This struct represents the standard message, which is used to send commands or information between different applications in the [MCManage network](https://github.com/Gooxey/MCManage.git). |

| Enum | Description |
|-|-|
| [MessageType](./src/message/message_type/mod.rs) | This enum describes the type of message holding this enum. |

| Error | Description |
|-|-|
| [MsgTypeError](./src/message/message_type/msg_type_error.rs) | This error type gets used by the [MessageType enum](./src/message/message_type/mod.rs). |

### Config

| Trait | Description |
|-|-|
| [ConfigTrait](./src/config_trait.rs) | Every struct implementing this trait can be used as the application's config. |

### MCServerManager

| Struct | Description |
|-|-|
| [MCServerManager](./src/mcserver_manager/mod.rs) | This struct is responsible for managing all [MCServers](./src/mcserver_manager/mcserver/mod.rs). ( starting, stopping, ... ) |

| Error | Description |
|-|-|
| [MCServerManagerError](./src/mcserver_manager/mcserver_manager_error.rs) |  Errors used by the [MCServerManager struct](./src/mcserver_manager/mod.rs). |

| Constant | Description |
|-|-|
| [SERVER_LIST_EXAMPLE_DEFAULT](./src/mcserver_manager/server_list_example_default.rs) | This constant represents the default text in the `servers/server_list_example.json` file. |

### MCServer

| Struct | Description |
|-|-|
| [MCServer](./src/mcserver_manager/mcserver/mod.rs)                        | This struct represents an API for one Minecraft server, which got assigned with the initiation of this struct. |
| [MCServerType](./src/mcserver_manager/mcserver/mcserver_type/mod.rs) | With this struct, the MCServer is able to interpret messages sent by a Minecraft server.                       |

| Enum | Description |
|-|-|
| [MCServerStatus](./src/mcserver_manager/mcserver/mcserver_status.rs) | This enum represents the [MCServer's](./src/mcserver_manager/mcserver/mod.rs) status. |

| Error | Description |
|-|-|
| [MCServerError](./src/mcserver_manager/mcserver/mcserver_error.rs)                             | Errors used by the [MCServer](./src/mcserver_manager/mcserver/mod.rs) struct.                        |
| [MCServerTypeError](./src/mcserver_manager/mcserver/mcserver_type/mcserver_type_error.rs) | Errors used by the [MCServerType](./src/mcserver_manager/mcserver/mcserver_type/mod.rs) struct. |

| Constant | Description |
|-|-|
| [MCSERVER_TYPES_DEFAULT](./src/mcserver_manager/mcserver/mcserver_type/mcserver_types_default.rs) | This constant represents the default text in the `config/mcserver_types.json` file. |

## Installation

Add the dependency to the `cargo.toml` file:

```text
[dependencies]
mcm_misc = { git = "https://github.com/Gooxey/mcm_misc.git", version = "X.Y.Z" }
    or
mcm_misc = { path = "/path/to/mcm_misc/" }
```

## Requirements

To use this library, [rust and cargo](https://www.rust-lang.org/tools/install) have to be installed.

## License

[GNU General Public License v3.0](https://choosealicense.com/licenses/gpl-3.0/)
