use chrono;


mod tests;


/// This function prints and/or saves a given string to the console or log file. A fancy mode will also be used if configured in the configuration of the application.
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
/// 
/// ## Example
/// 
/// ```
/// use mcm_misc::log::log;
/// 
/// # fn main() {    /// 
/// log("info", "r0", "Hello world!");
/// # }
/// ```
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