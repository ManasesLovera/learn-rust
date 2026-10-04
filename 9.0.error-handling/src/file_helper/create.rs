use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

#[allow(dead_code)]
pub fn open_or_create(path: &str) -> File {
    match File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == ErrorKind::NotFound => {
            File::create(path).unwrap_or_else(|e| panic!("Problem creating the file: {e:?}"))
        }
        Err(error) => panic!("Problem opening the file: {error:?}"),
    }
}

pub fn read_file_or_create(path: &str) -> Result<String, io::Error> {
    let result = File::open(path);

    let mut content_file = match result {
        Ok(file) => file,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            File::create(path)?;
            return Ok(String::new());
        }
        Err(e) => return Err(e),
    };

    let mut content = String::new();

    match content_file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(e),
    }
}
