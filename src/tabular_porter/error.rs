use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TabularPortError {
    NotInitialized(String),
    Unknown(String),
}

impl fmt::Display for TabularPortError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TabularPortError::NotInitialized(ref msg) => write!(f, "Not Initialized: {}", msg),
            TabularPortError::Unknown(ref msg) => write!(f, "Unknown: {}", msg),
        }
    }
}

impl Error for TabularPortError {}
