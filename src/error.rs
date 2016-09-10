//! Error module.

use std::fmt;
use std::error::Error;

/// From DTO Error
#[derive(Debug)]
pub struct FromDTOError {
    error: String,
}

impl FromDTOError {
    /// Creates a new FromDTOError
    pub fn new<S: AsRef<str>>(error: S) -> FromDTOError {
        FromDTOError { error: String::from(error.as_ref()) }
    }
}

impl fmt::Display for FromDTOError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl Error for FromDTOError {
    fn description(&self) -> &str {
        &self.error
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
