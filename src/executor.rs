use crate::{
    data::Data,
    io::{Input, Output},
};
use itertools::Itertools;

enum Instruction {
    Right(i32),
    Left(i32),
    Increment(u8),
    Decrement(u8),
    WriteOutput,
    ReadInput,
    StartLoop,
    EndLoop,
}

pub struct Executor {
    instructions: Vec<Instruction>,
    loop_targets: Vec<usize>,
}

impl Executor {
    pub fn new(code: &[u8]) -> Result<Self, Error> {
        let mut index = 0;
        let mut starts = vec![];
        let mut instructions = Vec::with_capacity(code.len());
        let mut loop_targets = vec![0; code.len()];

        for (repeats, ch) in code.iter().dedup_with_count() {
            match *ch {
                b'>' => instructions.push(Instruction::Right(repeats as i32)),
                b'<' => instructions.push(Instruction::Left(repeats as i32)),
                b'+' => instructions.push(Instruction::Increment(repeats as u8)),
                b'-' => instructions.push(Instruction::Decrement(repeats as u8)),
                b'.' => (0..repeats).for_each(|_| instructions.push(Instruction::WriteOutput)),
                b',' => (0..repeats).for_each(|_| instructions.push(Instruction::ReadInput)),
                b'[' => (0..repeats).for_each(|_| {
                    starts.push(instructions.len());
                    instructions.push(Instruction::StartLoop);
                }),
                b']' => {
                    for i in 0..repeats {
                        let start = starts
                            .pop()
                            .ok_or_else(|| Error::UnmatchedLoopEnd(index + i))?;
                        loop_targets[start] = instructions.len();
                        loop_targets[instructions.len()] = start;
                        instructions.push(Instruction::EndLoop);
                    }
                }
                _ => {} // Some sort of comment - ignore
            };
            index += repeats;
        }

        loop_targets.truncate(instructions.len());

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
                Instruction::Right(n) => {
                    data_index += n;
                }
                Instruction::Left(n) => {
                    data_index -= n;
                }
                Instruction::Increment(n) => {
                    *data.get(data_index) += n;
                }
                Instruction::Decrement(n) => {
                    *data.get(data_index) -= n;
                }
                Instruction::WriteOutput => {
                    output.write(*data.get(data_index));
                }
                Instruction::ReadInput => {
                    *data.get(data_index) = input.read().map_err(|err| super::Error::Input(err))?;
                }
                Instruction::StartLoop => {
                    if *data.get(data_index) == 0 {
                        inst_index = self.loop_targets[inst_index];
                    }
                }
                Instruction::EndLoop => {
                    if *data.get(data_index) != 0 {
                        inst_index = self.loop_targets[inst_index];
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
