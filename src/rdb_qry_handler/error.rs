use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum QueryHandleError {
    NotInitialized(String),
    InvalidCall(String),
    Unknown(String),
}

impl fmt::Display for QueryHandleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            QueryHandleError::NotInitialized(ref msg) => write!(f, "Not Initialized: {}", msg),
            QueryHandleError::InvalidCall(ref msg) => write!(f, "Invalid Call: {}", msg),
            QueryHandleError::Unknown(ref msg) => write!(f, "Unknown: {}", msg),
        }
    }
}

impl Error for QueryHandleError {}
