use ansi_term::Color;
use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum AppError {
    IoError(io::Error),
    SerdeError(serde_json::Error),
    InvalidDirectory(PathBuf),
    FileNotFound(String),
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::IoError(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::SerdeError(err)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_prefix = Color::Red.bold().paint("error:");

        match self {
            AppError::IoError(err) => write!(f, "{} I/O error: {}", error_prefix, err),
            AppError::SerdeError(err) => {
                write!(f, "{} Serialization error: {}", error_prefix, err)
            }
            AppError::InvalidDirectory(dir) => {
                write!(f, "{} Directory `{:?}` does not exist", error_prefix, dir)
            }
            AppError::FileNotFound(dir) => {
                write!(f, "{} File `{}` not found", error_prefix, dir)
            }
        }
    }
}
