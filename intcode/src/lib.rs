use std::slice::SliceIndex;

#[derive(Debug)]
pub struct IntcodeInterpreter {
    inputs: Vec<i32>,
    outputs: Vec<i32>,
    code: Vec<i32>,
    current_position: usize,
}

#[derive(Debug)]
pub enum IntcodeError {
    UnknownOpcode { opcode: i32, index: usize },
    InvalidTargetMode,
    IndexOutOfBounds,
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
    Input,
    Output,
    End,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Debug)]
struct Opcode {
    operation: Operation,
    parameter_modes: [ParameterMode; 3],
}

impl From<i32> for Opcode {
    fn from(mut value: i32) -> Self {
        let operation = match value % 100 {
            1 => Operation::Add,
            2 => Operation::Multiply,
            3 => Operation::Input,
            4 => Operation::Output,
            99 => Operation::End,
            _ => Operation::Unknown,
        };

        let mut parameter_modes = [ParameterMode::Position; 3];

        value /= 100;
        let mut i = 0;

        while value > 0 {
            parameter_modes[i] = if value & 1 == 1 {
                ParameterMode::Immediate
            } else {
                ParameterMode::Position
            };

            i += 1;
            value /= 10;
        }

        Self {
            operation,
            parameter_modes,
        }
    }
}

impl Opcode {
    fn parameters_count(&self) -> usize {
        match self.operation {
            Operation::Add | Operation::Multiply => 3,
            Operation::Input | Operation::Output => 1,
            Operation::End | Operation::Unknown => 0,
        }
    }
}

#[derive(Debug)]
struct Parameter {
    mode: ParameterMode,
    value: i32,
}

impl Parameter {
    pub fn get(&self, code: &Vec<i32>) -> Result<i32, IntcodeError> {
        match self.mode {
            ParameterMode::Immediate => Ok(self.value),
            ParameterMode::Position => code.get(self.value as usize).map(|v| *v).ok_or(IntcodeError::IndexOutOfBounds),
        }
    }

    pub fn set(&self, code: &mut Vec<i32>, value: i32) -> Result<(), IntcodeError> {
        match self.mode {
            ParameterMode::Immediate => Err(IntcodeError::InvalidTargetMode),
            ParameterMode::Position => {
                *(code
                    .get_mut(self.value as usize)
                    .ok_or(IntcodeError::IndexOutOfBounds)?) = value;

                Ok(())
            }
        }
    }
}

impl IntcodeInterpreter {
    fn get<T>(&self, idx: T) -> Result<&T::Output, IntcodeError>
    where
        T: SliceIndex<[i32]>,
    {
        self.code.get(idx).ok_or(IntcodeError::IndexOutOfBounds)
    }

    pub fn get_code(&self) -> &Vec<i32> {
        &self.code
    }

    pub fn run(&mut self) -> Result<(), IntcodeError> {
        let mut inputs_iter = self.inputs.iter();

        loop {
            let next_code = *self.get(self.current_position)?;
            let opcode: Opcode = next_code.into();

            let parameters_count = opcode.parameters_count();
            let parameters: Vec<_> = self
                .get(self.current_position + 1..=self.current_position + parameters_count)?
                .into_iter()
                .zip(opcode.parameter_modes.into_iter())
                .map(|(value, param_mode)| {
                    Parameter {
                        value: *value,
                        mode: *param_mode,
                    }
                })
                .collect();

            macro_rules! unwrap_slice {
                ($pattern: pat => $action: block) => {
                    match parameters.as_slice() {
                        $pattern => $action,
                        _ => unreachable!(),
                    }
                };
            }

            match opcode.operation {
                Operation::Add => {
                    unwrap_slice!([a, b, target] => {
                        let a = a.get(&self.code)?;
                        let b = b.get(&self.code)?;

                        target.set(&mut self.code, a + b)?
                    });
                },
                Operation::Multiply => {
                    unwrap_slice!([a, b, target] => {
                        let a = a.get(&self.code)?;
                        let b = b.get(&self.code)?;

                        target.set(&mut self.code, a * b)?
                    });
                },
                Operation::Input => {
                    unwrap_slice!([target] => {
                        target.set(&mut self.code, *inputs_iter.next().ok_or(IntcodeError::IndexOutOfBounds)?)?
                    });
                },
                Operation::Output => {
                    unwrap_slice!([source] => {
                        self.outputs.push(source.get(&mut self.code)?);
                    });
                }
                Operation::End => break Ok(()),
                Operation::Unknown => {
                    return Err(IntcodeError::UnknownOpcode {
                        opcode: next_code,
                        index: self.current_position,
                    })
                },
            }

            self.current_position += parameters_count + 1;
        }
    }
}

impl From<Vec<i32>> for IntcodeInterpreter {
    fn from(code: Vec<i32>) -> Self {
        Self {
            inputs: Vec::new(),
            outputs: Vec::new(),
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
