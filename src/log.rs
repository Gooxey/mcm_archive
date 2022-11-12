use chrono;

/// Write a log message to the Terminal.
/// 
/// # Parameters
/// 
/// `msg_kind: &str` -> The kind of message ( info, warn, erro ) \
/// `sender: &str` -> The callers id. ( Proxy, Console ... ) \
/// `message: &str` -> The Message to print to the Console.
/// 
/// # Returns
/// 
/// `0` -> Log written successfully.
/// `1` -> msg_kind invalid.
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

#[cfg(test)]
mod tests {
    use super::*;
  
    #[test]
    fn test2() {
        log("info", "Proxy", "hello");
        assert!(true);
    }
}