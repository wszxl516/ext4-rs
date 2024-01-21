use alloc::string::String;

#[derive(Debug)]
pub enum Error {
    InvalidInput(String),
    NotFound(String),
    IOError(String),
    UnexpectedEof(String),
    InvalidData(String),
    FileExists(String),
}