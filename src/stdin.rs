use std::{
    error::Error,
    fmt::Display,
    io::{stdin, BufReader, Read},
};

#[derive(Debug)]
pub struct StdinReadError;

impl Error for StdinReadError {}

impl Display for StdinReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to read from stdin.")
    }
}

pub fn read_from_stdin() -> String {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut buffer: Vec<u8> = Vec::new();

    reader.read_to_end(&mut buffer).expect("Failed to read from stdin.");

    String::from(std::str::from_utf8(&buffer).expect("Failed to convert stdin to string."))
}
