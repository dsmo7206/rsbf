use std::io::{Bytes, Read, Stdin};

pub struct StdinInput {
    bytes: Bytes<Stdin>,
}

impl StdinInput {
    pub fn new() -> Self {
        Self {
            bytes: std::io::stdin().bytes(),
        }
    }
}

impl Default for StdinInput {
    fn default() -> Self {
        Self::new()
    }
}

impl super::Input for StdinInput {
    type ErrorType = StdinError;

    fn read(&mut self) -> Result<u8, Self::ErrorType> {
        self.bytes
            .next()
            .ok_or(StdinError::NoMoreInput)?
            .map_err(StdinError::IoError)
    }
}

#[derive(Debug)]
pub enum StdinError {
    IoError(std::io::Error),
    NoMoreInput,
}

impl std::error::Error for StdinError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IoError(inner) => Some(inner),
            Self::NoMoreInput => None,
        }
    }
}

impl std::fmt::Display for StdinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(inner) => write!(f, "{}", inner),
            Self::NoMoreInput => write!(f, "No more input"),
        }
    }
}

pub struct StdoutOutput {}

impl super::Output for StdoutOutput {
    fn write(&mut self, byte: u8) {
        print!("{}", byte as char);
    }
}
