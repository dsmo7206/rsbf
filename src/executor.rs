use crate::{
    data::Data,
    io::{Input, Output},
};
use std::collections::HashMap;

enum Instruction {
    Right,
    Left,
    Increment,
    Decrement,
    WriteOutput,
    ReadInput,
    StartLoop,
    EndLoop,
}

pub struct Executor {
    instructions: Vec<Instruction>,
    loop_targets: HashMap<usize, usize>,
}

impl Executor {
    pub fn new(code: &[u8]) -> Result<Self, Error> {
        let mut starts = vec![];
        let mut instructions = Vec::with_capacity(code.len());
        let mut loop_targets = HashMap::new();

        for (index, ch) in code.iter().enumerate() {
            match *ch {
                b'>' => instructions.push(Instruction::Right),
                b'<' => instructions.push(Instruction::Left),
                b'+' => instructions.push(Instruction::Increment),
                b'-' => instructions.push(Instruction::Decrement),
                b'.' => instructions.push(Instruction::WriteOutput),
                b',' => instructions.push(Instruction::ReadInput),
                b'[' => {
                    starts.push(instructions.len());
                    instructions.push(Instruction::StartLoop);
                }
                b']' => {
                    let start = starts.pop().ok_or_else(|| Error::UnmatchedLoopEnd(index))?;
                    loop_targets.insert(start, instructions.len());
                    loop_targets.insert(instructions.len(), start);
                    instructions.push(Instruction::EndLoop);
                }
                _ => {} // Some sort of comment - ignore
            };
        }

        if let Some(start) = starts.pop() {
            Err(Error::UnmatchedLoopStart(start))
        } else {
            Ok(Self {
                instructions,
                loop_targets,
            })
        }
    }

    pub fn run<D, I, O>(
        &self,
        data: &mut D,
        input: &mut I,
        output: &mut O,
    ) -> Result<usize, super::Error<I::ErrorType>>
    where
        D: Data,
        I: Input,
        O: Output,
    {
        let mut data_index = 0i32;
        let mut inst_index = 0;
        let mut num_insts = 0;

        while inst_index < self.instructions.len() {
            match self.instructions[inst_index] {
                Instruction::Right => {
                    data_index += 1;
                }
                Instruction::Left => {
                    data_index -= 1;
                }
                Instruction::Increment => {
                    *data.get(data_index) += 1;
                }
                Instruction::Decrement => {
                    *data.get(data_index) -= 1;
                }
                Instruction::WriteOutput => {
                    output.write(*data.get(data_index));
                }
                Instruction::ReadInput => {
                    *data.get(data_index) = input.read().map_err(|err| super::Error::Input(err))?;
                }
                Instruction::StartLoop => {
                    if *data.get(data_index) == 0 {
                        inst_index = *self.loop_targets.get(&inst_index).unwrap();
                    }
                }
                Instruction::EndLoop => {
                    if *data.get(data_index) != 0 {
                        inst_index = *self.loop_targets.get(&inst_index).unwrap();
                    }
                }
            }
            inst_index += 1;
            num_insts += 1;
        }

        Ok(num_insts)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unmatched end loop at offset {0}")]
    UnmatchedLoopEnd(usize),
    #[error("Unmatched start loop at offset {0}")]
    UnmatchedLoopStart(usize),
}
