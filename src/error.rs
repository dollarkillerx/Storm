use self::StormError::*;
use std::fmt;
use serde::export::Formatter;
use std::error::Error;

#[derive(Debug)]
pub enum StormError {
    SocketBindErr,
}

impl fmt::Display for StormError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            SocketBindErr => write!(f, "Socket Bind Error"),
        }
    }
}

impl Error for StormError {
    fn description(&self) -> &str {
        match *self {
            SocketBindErr => "Socket Bind Err",
        }
    }
}