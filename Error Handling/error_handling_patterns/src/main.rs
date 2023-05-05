use std::{fmt, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Invalid value: {0}")]
    InvalidValue(String),
}

fn main() -> Result<(), MyError> {
    let value = "invalid";
    if value == "invalid" {
        return Err(MyError::InvalidValue(value.into()));
    }
    Ok(())
}
