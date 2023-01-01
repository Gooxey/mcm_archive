//! This module provided the log function. It can be used to print and save a given string to a file or the console. This can be done in a fancy mode (colored text) if enabled
//! by the [`application's config`](crate::config::Config).


use chrono;


mod tests;


/// This function can be used to print and save a given string to a file or the console. This can be done in a fancy mode (colored text) if enabled
/// by the [`application's config`](crate::config::Config).
/// 
/// ## Parameters
/// 
/// | Parameter        | Description                              |
/// |------------------|------------------------------------------|
/// | `msg_kind: &str` | The kind of message ( info, warn, erro ) |
/// | `sender: &str`   | The callers id. ( Proxy, Console ... )   |
/// | `message: &str`  | The message to log.                      |
/// 
/// ## Returns
/// 
/// | Parameter | Description                       |
/// |-----------|-----------------------------------|
/// | `0`       | The log was written successfully. |
/// | `1`       | msg_kind invalid.                 |
pub fn log(msg_kind: &str, sender: &str, message: &str) -> i32 {
    let info = "\x1b[34m\x1b[1mINFO\x1b[0m |";
    let warn = "\x1b[33m\x1b[1mWARN\x1b[0m |";
    let erro = "\x1b[31m\x1b[1mERRO\x1b[0m |";

    let msg_kind_text;
    match msg_kind {
        "info" => { msg_kind_text = info; }
        "warn" => { msg_kind_text = warn; }
        "erro" => { msg_kind_text = erro; }
        _ => { return 1 },
    }

    let time_stamp = chrono::Local::now().format("\x1b[2m\x1b[1m%d.%m.%Y\x1b[0m | \x1b[2m\x1b[1m%H:%M:%S\x1b[0m |");

    println!("{} {} {} | {}", time_stamp, msg_kind_text, sender, message);
    return 0;
}