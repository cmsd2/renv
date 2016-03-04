
use std::result;
use std::io;

#[derive(Debug,Clone,PartialEq)]
pub enum Error {
    Error(String),
    StringError,
    IoError(io::ErrorKind),
    FileNotFound(String),
    Killed
}

pub type Result<T> = result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(io_err: io::Error) -> Error {
        Error::IoError(io_err.kind())
    }
}
