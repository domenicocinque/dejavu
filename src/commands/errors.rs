use std::io;

#[derive(Debug)]
#[allow(dead_code)]
pub enum DeduplicationError {
    IoError(io::Error),
    SerdeError(serde_json::Error),
    InvalidDirectory(String), // Custom error for invalid directories
}

// todo: implement display

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
