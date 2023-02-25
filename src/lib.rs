//! Functions, structs, and more used by applications in the [`MCManage Network`](https://github.com/Gooxey/MCManage.git). \
//! \
//! This is part of the [`MCManage`](https://github.com/Gooxey/MCManage.git) project, which tries to make it as easy as possible to create and manage your [`Minecraft servers`](https://www.minecraft.net).
//!  
//! 
//! # Installation
//! 
//! Add the dependency to the `cargo.toml` file:
//! ```text
//! [dependencies]
//! mcm_misc = { git = "https://github.com/Gooxey/mcm_misc.git", version = "X.Y.Z" }
//! ```


#![warn(missing_docs)]


pub mod qol;
pub mod message;
pub mod mcserver_manager;
pub mod concurrent_class;
pub mod mcmanage_error;
pub mod config;

mod test_functions;