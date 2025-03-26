use serde::Serialize;
use std::{error::Error, fmt::Display};

#[derive(Debug, Serialize)]
pub struct FetchError {
    pub message: String,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FetchError: {}", self.message)
    }
}

impl Error for FetchError {}
