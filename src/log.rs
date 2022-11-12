use chrono;

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
    let info = "\033[34m\033[1mINFO\033[0m |";
    let warn = "\033[33m\033[1mWARN\033[0m |";
    let erro = "\033[31m\033[1mERRO\033[0m |";

    let msg_kind_text;
    match msg_kind {
        "info" => { msg_kind_text = info; }
        "warn" => { msg_kind_text = warn; }
        "erro" => { msg_kind_text = erro; }
        _ => { return 1 },
    }

    let time_stamp = chrono::Local::now().format("\033[2m\033[1m%d.%m.%Y\033[0m | \033[2m\033[1m%H:%M:%S\033[0m |");

    println!("{} {} {} | {}", time_stamp, msg_kind_text, sender, message);
    return 0;
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;
  
    #[test]
    fn log__normal() {
        match log("info", "Proxy", "hello") {
            0 => {
                assert!(true);
            }
            1 => {
                assert!(true, "An invalid type error got returned.")
            }
            _ => {
                assert!(false, "The function is not supposed to return any numbers except 0 and 1.")
            }
        }
    }
    #[test]
    fn log__invalid_type() {
        match log("invalid", "Proxy", "hello") {
            0 => {
                assert!(false, "Expected function to throw a invalid type error.");
            }
            1 => {
                assert!(true)
            }
            _ => {
                assert!(false, "The function is not supposed to return any numbers except 0 and 1.")
            }
        }
    }
    #[test]
    fn log__space_text() {
        match log("info", " ", " ") {
            0 => {
                assert!(true);
            }
            1 => {
                assert!(true, "An invalid type error got returned.")
            }
            _ => {
                assert!(false, "The function is not supposed to return any numbers except 0 and 1.")
            }
        }
    }
}