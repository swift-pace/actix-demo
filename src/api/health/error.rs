use std::{convert::From, fmt::Display};

pub enum Error {
    InvalidRequest,
    IO(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let message = match self {
            Self::InvalidRequest => "Invalid Request",
            Self::IO(e) => e,
        };
        write!(f, "Error: {}", message)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e.to_string())
    }
}
