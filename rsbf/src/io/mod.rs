mod stdinout;

pub use stdinout::{StdinError, StdinInput, StdoutOutput};

pub trait Input {
    type ErrorType: std::error::Error;

    fn read(&mut self) -> Result<u8, Self::ErrorType>;
}

pub trait Output {
    fn write(&mut self, byte: u8);
}
