use std::result;
use super::error;

pub type Result<T> = result::Result<T, error::Error>;
