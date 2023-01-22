//! This module provides the log macro. It can be used to print and save a given string to the log file and the console.
//! 
//! ## Macros
//! 
//! | Macro                           | Description                                                                       |
//! |---------------------------------|-----------------------------------------------------------------------------------|
//! | [log_print!](crate::log_print!) | This macro can be used to print a given string to the console.                    |
//! | [log!](crate::log!)             | This macro can be used to print and save a given string to a file or the console. |

pub extern crate chrono;

/// This macro can be used to print a given string to the console.
/// 
/// ## Parameters
/// 
/// 1. This represents the `variant` of the log. There are three states:
///     1. warn => Use this one in case you want to warn the user about something.
///     2. erro => In the event of an error, use this one. 
///     3. info => This one is the default, so no specific input is required.
/// 
/// 2. This is the `name` under which this log should be sent. ( The maximum length is `16 characters`. Everything above will be cut off. )
/// 
/// 3. The following arguments represent the `message` to be sent. It can be used in the same way as the [`format! macro`](format!).
/// 
/// ## Example
/// 
/// ```rust
/// # use mcm_misc::mcserver::mcserver_error::MCServerError;
/// # use mcm_misc::log;
/// let err = MCServerError::FatalError;
/// 
/// log!("erro", "MyFirstMCServer", "An error occurred while waiting on the Minecraft server to finish. Error: {}", err);
/// ```
#[macro_export]
macro_rules! log_print {
    ($variant: expr, $sender: expr, $( $arguments: tt ) *) => {
        print!("{} | ", $crate::log::chrono::Local::now().format("\x1b[2m\x1b[1m%d.%m.%Y\x1b[0m | \x1b[2m\x1b[1m%H:%M:%S\x1b[0m"));
        print!("{} | ", 
            match $variant {
                "warn" => "\x1b[93m\x1b[1mWARN\x1b[0m",
                "erro" => "\x1b[91m\x1b[1mERRO\x1b[0m",
                _ => "\x1b[94m\x1b[1mINFO\x1b[0m" // the default is an info text
            }
        );
        print!("\x1b[97m\x1b[1m{:<16.16}\x1b[0m | ", $sender);
        print!($ ( $arguments ) *);
        print!("\n");
    };
}


/// This macro can be used to print and save a given string to a file or the console.
/// 
/// ## Parameters
/// 
/// 1. This represents the `variant` of the log. There are three states:
///     1. warn => Use this one in case you want to warn the user about something.
///     2. erro => In the event of an error, use this one. 
///     3. info => This one is the default, so no specific input is required.
/// 
/// 2. This is the `name` under which this log should be sent. ( The maximum length is `16 characters`. Everything above will be cut off. )
/// 
/// 3. The following arguments represent the `message` to be sent. It can be used in the same way as the [`format! macro`](format!).
/// 
/// ## Example
/// 
/// ```rust
/// # use mcm_misc::mcserver::mcserver_error::MCServerError;
/// # use mcm_misc::log;
/// let err = MCServerError::FatalError;
/// 
/// log!("erro", "MyFirstMCServer", "An error occurred while waiting on the Minecraft server to finish. Error: {}", err);
/// ```
#[macro_export]
macro_rules! log {
    ($variant: expr, $sender: expr, $( $arguments: tt ) *) => {       
        $crate::log_print!($variant, $sender, $( $arguments ) *);

        let mut log: String = "".to_string();
        log += &format!("{} | ", $crate::log::chrono::Local::now().format("%d.%m.%Y | %H:%M:%S"));
        log += &format!("{} | ", 
            match $variant {
                "warn" => "WARN",
                "erro" => "ERRO",
                _ => "INFO" // the default is an info text
            }
        );
        log += &format!("{:<16.16} | ", $sender);
        log += &format!($ ( $arguments ) *);
        log += &format!("\n");

        match std::fs::File::options().append(true).create_new(true).open("logs/log.txt") {
            Ok(mut log_file) => {
                loop {
                    if let Ok(_) = std::io::Write::write_all(&mut log_file, log.as_bytes()) {
                        break;
                    }
                }
            }
            Err(erro) => {
                match erro.kind() {
                    std::io::ErrorKind::NotFound => {
                        std::fs::create_dir("logs").unwrap(); // no error is expected, so we unwrap here

                        let mut log_file = std::fs::File::options().append(true).create_new(true).open("logs/log.txt").unwrap(); // no error is expected, so we unwrap here
                        loop {
                            if let Ok(_) = std::io::Write::write_all(&mut log_file, log.as_bytes()) {
                                break;
                            }
                        }
                    }
                    std::io::ErrorKind::AlreadyExists => {                        
                        let mut log_file = std::fs::File::options().append(true).open("logs/log.txt").unwrap(); // no error is expected, so we unwrap here
                        loop {
                            if let Ok(_) = std::io::Write::write_all(&mut log_file, log.as_bytes()) {
                                break;
                            }
                        }
                    }
                    _ => {
                        panic!("An unhandled error occurred while writing a log to the log file.")
                    }
                }
            }
        }
    }
}