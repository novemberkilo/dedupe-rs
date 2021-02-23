use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    ErrorMessage(String),
    IOError(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ErrorMessage(e) =>
                write!(f, "Error: {}", e),
            Error::IOError(e) =>
                write!(f, "IO Error: {}", e), 
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileData {
    pub path: std::path::PathBuf,
} 
