
use std::error::Error;
use std::result;
use std::io;
use java_properties;

#[derive(Debug,Clone)]
pub enum RsEnvError {
    Error(String),
    StringError,
    IoError(io::ErrorKind),
    FileNotFound(String),
    Killed,
    ChildExited(i32),
    PropertiesError(String, Option<usize>),
}

pub type Result<T> = result::Result<T, RsEnvError>;

impl From<io::Error> for RsEnvError {
    fn from(io_err: io::Error) -> RsEnvError {
        RsEnvError::IoError(io_err.kind())
    }
}

impl From<java_properties::PropertiesError> for RsEnvError {
    fn from(p_err: java_properties::PropertiesError) -> RsEnvError {
        RsEnvError::PropertiesError(p_err.description().to_owned(), p_err.line_number())
    }
}
