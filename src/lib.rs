mod data;
mod executor;
mod io;

pub use data::{Data, ResizableData};
pub use executor::Executor;
pub use io::{Input, Output, StdinError, StdinInput, StdoutOutput};

pub fn run_simple(code: &[u8]) -> Result<(), Error<StdinError>> {
    let mut data = ResizableData::new();
    let mut input = StdinInput::new();
    let mut output = StdoutOutput {};
    let executor = Executor::new(code).map_err(|err| Error::Executor(err))?;

    executor.run(&mut data, &mut input, &mut output)?;

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum Error<InputError>
where
    InputError: std::error::Error,
{
    #[error("Executor error: {0}")]
    Executor(executor::Error),
    #[error("Input error: {0}")]
    Input(InputError),
}
