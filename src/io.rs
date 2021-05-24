use std::io::{Bytes, Read, Stdin};

pub trait Input {
    type ErrorType: std::error::Error;

    fn read(&mut self) -> Result<u8, Self::ErrorType>;
}

pub trait Output {
    fn write(&mut self, byte: u8);
}

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

impl Input for StdinInput {
    type ErrorType = StdinError;

    fn read(&mut self) -> Result<u8, Self::ErrorType> {
        self.bytes
            .next()
            .ok_or_else(|| StdinError::NoMoreInput)?
            .map_err(|_| StdinError::NoMoreInput)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum StdinError {
    #[error("No more input")]
    NoMoreInput,
}

pub struct StdoutOutput {}

impl Output for StdoutOutput {
    fn write(&mut self, byte: u8) {
        print!("{}", byte as char);
    }
}
