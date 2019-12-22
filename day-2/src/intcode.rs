use std::slice::SliceIndex;

#[derive(Debug)]
pub struct IntcodeInterpreter {
    code: Vec<u32>,
    current_position: usize,
}

#[derive(Debug)]
pub enum IntcodeError {
    UnknownOpcode { opcode: u32, index: usize },
    IndexOutOfBounds,
}

#[derive(Debug)]
enum Opcode {
    Add,
    Multiply,
    End,
    Unknown,
}

impl From<u32> for Opcode {
    fn from(value: u32) -> Self {
        match value {
            1 => Opcode::Add,
            2 => Opcode::Multiply,
            99 => Opcode::End,
            _ => Opcode::Unknown,
        }
    }
}

impl IntcodeInterpreter {
    fn set(&mut self, idx: usize, opcode: u32) -> Result<(), IntcodeError> {
        *(self
            .code
            .get_mut(idx)
            .ok_or(IntcodeError::IndexOutOfBounds)?) = opcode;

        Ok(())
    }

    fn get<T>(&self, idx: T) -> Result<&T::Output, IntcodeError>
    where
        T: SliceIndex<[u32]>,
    {
        self.code.get(idx).ok_or(IntcodeError::IndexOutOfBounds)
    }

    pub fn get_code(&self) -> &Vec<u32> {
        &self.code
    }

    pub fn run(&mut self) -> Result<(), IntcodeError> {
        loop {
            let opcode = *self.get(self.current_position)?;

            match opcode.into() {
                Opcode::Add => {
                    let operands =
                        self.get(self.current_position + 1..self.current_position + 4)?;

                    match operands {
                        [a, b, target] => {
                            let sum = self.get(*a as usize)? + self.get(*b as usize)?;
                            let target = *target as usize;

                            self.set(target, sum)?
                        }
                        _ => unreachable!(),
                    }
                }
                Opcode::Multiply => {
                    let operands =
                        self.get(self.current_position + 1..self.current_position + 4)?;

                    match operands {
                        [a, b, target] => {
                            let product = self.get(*a as usize)? * self.get(*b as usize)?;
                            let target = *target as usize;

                            self.set(target, product)?
                        }
                        _ => unreachable!(),
                    }
                }
                Opcode::End => break Ok(()),
                Opcode::Unknown => {
                    return Err(IntcodeError::UnknownOpcode {
                        opcode: opcode as u32,
                        index: self.current_position,
                    })
                }
            }

            self.current_position += 4;
        }
    }
}

impl From<Vec<u32>> for IntcodeInterpreter {
    fn from(code: Vec<u32>) -> Self {
        Self {
            code,
            current_position: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        macro_rules! test_intcode {
            ($input: expr, $expected: expr) => {
                let mut interpreter: IntcodeInterpreter = $input.to_vec().into();
                interpreter.run().unwrap();

                let result = interpreter.get_code();
                assert_eq!(result[..], $expected[..]);
            };
        }

        test_intcode!([1, 0, 0, 0, 99], [2, 0, 0, 0, 99]);

        test_intcode!([2, 3, 0, 3, 99], [2, 3, 0, 6, 99]);

        test_intcode!([2, 4, 4, 5, 99, 0], [2, 4, 4, 5, 99, 9801]);

        test_intcode!([1, 1, 1, 4, 99, 5, 6, 0, 99], [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
