use ansi_term::Color;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum DeduplicationError {
    IoError(io::Error),
    SerdeError(serde_json::Error),
    InvalidDirectory(String),
}

impl From<io::Error> for DeduplicationError {
    fn from(err: io::Error) -> Self {
        DeduplicationError::IoError(err)
    }
}

impl From<serde_json::Error> for DeduplicationError {
    fn from(err: serde_json::Error) -> Self {
        DeduplicationError::SerdeError(err)
    }
}

impl fmt::Display for DeduplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_prefix = Color::Red.bold().paint("error:");

        match self {
            DeduplicationError::IoError(err) => write!(f, "{} I/O error: {}", error_prefix, err),
            DeduplicationError::SerdeError(err) => {
                write!(f, "{} Serialization error: {}", error_prefix, err)
            }
            DeduplicationError::InvalidDirectory(dir) => {
                write!(f, "{} Directory `{}` does not exist", error_prefix, dir)
            }
        }
    }
}
