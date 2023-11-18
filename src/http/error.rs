use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

pub struct Error { pub message: &'static str }

impl Debug for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("Error").finish()
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("Error").finish()
    }
}

impl error::Error for Error {  }
