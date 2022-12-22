use std::fs::File;
use std::io::{Read, BufReader};


mod tests;


pub enum Command {
    /// Return the contents of a file specified. The path supplied will start at the applications root dictionary.
    GetFile(String)
}
impl Command {
    /// Execute the command of this enum and return its result.
    pub fn execute(&self) -> Result<String, std::io::Error> {
        match self {
            Self::GetFile(filepath) => { Self::getfile(filepath) }
        }
    }
    /// Get a string version of this enum variant. The data held by the variant will not be described by this string.
    pub fn to_string(&self) -> String {
        match self {
            Self::GetFile(_) => { "getfile".to_owned() }
        }
    }

    /// Return the contents of a file specified. The path supplied will start at the applications root dictionary.
    /// 
    /// ## Parameters
    /// 
    /// | Parameter           | Description                                 |
    /// |---------------------|---------------------------------------------|
    /// | `filepath: &String` | The path of the file which needs to be read |
    fn getfile(filepath: &String) -> Result<String, std::io::Error> {
        let file;
        match File::open(filepath) {
            Ok(f) => { file = f; }
            Err(err) => {
                return Err(err)
            }
        }
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        match buf_reader.read_to_string(&mut contents) {
            Ok(_) => { /* File was read successfully */ }
            Err(err) => {
                return Err(err)
            }
        }
        Ok(contents)
    }
}