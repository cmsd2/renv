
use std::error::Error;
use std::result;
use std::io;
use java_properties;

#[derive(Debug,Clone)]
pub enum REnvError {
    Error(String),
    StringError,
    IoError(io::ErrorKind),
    FileNotFound(String),
    FileExists(String),
    Killed,
    ChildExited(i32),
    PropertiesError(String, Option<usize>),
}

pub type Result<T> = result::Result<T, REnvError>;

impl From<io::Error> for REnvError {
    fn from(io_err: io::Error) -> REnvError {
        REnvError::IoError(io_err.kind())
    }
}

impl From<java_properties::PropertiesError> for REnvError {
    fn from(p_err: java_properties::PropertiesError) -> REnvError {
        REnvError::PropertiesError(p_err.description().to_owned(), p_err.line_number())
    }
}
