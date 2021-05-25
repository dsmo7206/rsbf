mod data;
mod executor;
mod io;

pub use data::{ArrayData, Data, ResizableData};
pub use executor::Executor;
pub use io::{Input, Output, StdinError, StdinInput, StdoutOutput};

use std::error::Error as StdError;

pub fn run_simple(code: &[u8]) -> Result<usize, Error<StdinError>> {
    //let mut data = ResizableData::new();
    let mut data = data::ArrayData::new();
    let mut input = StdinInput::new();
    let mut output = StdoutOutput {};
    let executor = Executor::new(code).map_err(Error::Executor)?;

    executor.run(&mut data, &mut input, &mut output)
}

#[derive(Debug)]
pub enum Error<InputError>
where
    InputError: StdError + 'static,
{
    Executor(executor::Error),
    Input(InputError),
}

impl<InputError> StdError for Error<InputError>
where
    InputError: StdError,
{
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Executor(inner) => Some(inner),
            Self::Input(inner) => Some(inner),
        }
    }
}

impl<InputError> std::fmt::Display for Error<InputError>
where
    InputError: StdError,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Executor(inner) => write!(f, "{}", inner),
            Self::Input(inner) => write!(f, "{}", inner),
        }
    }
}
