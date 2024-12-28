use std::io;

#[derive(Debug)]
#[allow(dead_code)]
pub enum DuplicationError {
    IoError(io::Error),
    SerdeError(serde_json::Error),
    InvalidDirectory(String), // Custom error for invalid directories
}

// todo: implement display

impl From<io::Error> for DuplicationError {
    fn from(err: io::Error) -> Self {
        DuplicationError::IoError(err)
    }
}

impl From<serde_json::Error> for DuplicationError {
    fn from(err: serde_json::Error) -> Self {
        DuplicationError::SerdeError(err)
    }
}
