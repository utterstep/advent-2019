mod opcode;

use opcode::{Opcode, Operation, ParameterMode};

#[derive(Debug)]
struct Parameter {
    mode: ParameterMode,
    value: i32,
}

impl Parameter {
    /// Creates iterator over params from opcode and code slice, starting from current element
    ///
    /// Returns iterator instead of vec to avoid allocation (~3x performance penalty)
    fn from_opcode<'a>(
        opcode: &'a Opcode,
        code: &'a [i32],
    ) -> Result<impl Iterator<Item = Parameter> + 'a, IntcodeError> {
        let parameters_count = opcode.parameters_count();

        Ok(code
            .get(..parameters_count)
            .ok_or(IntcodeError::IndexOutOfBounds)?
            .iter()
            .zip(opcode.parameter_modes.iter())
            .map(|(&value, &mode)| Parameter { value, mode }))
    }

    #[inline]
    pub fn read(&self, code: &[i32]) -> Result<i32, IntcodeError> {
        match self.mode {
            ParameterMode::Immediate => Ok(self.value),
            ParameterMode::Position => code
                .get(self.value as usize)
                .copied()
                .ok_or(IntcodeError::IndexOutOfBounds),
        }
    }

    #[inline]
    pub fn write(&self, code: &mut [i32], value: i32) -> Result<(), IntcodeError> {
        match self.mode {
            ParameterMode::Immediate => Err(IntcodeError::WriteToConstantProhibited),
            ParameterMode::Position => {
                *(code
                    .get_mut(self.value as usize)
                    .ok_or(IntcodeError::IndexOutOfBounds)?) = value;

                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub struct IntcodeInterpreter {
    input: Vec<i32>,
    code: Vec<i32>,
    current_position: usize,
}

#[derive(Debug)]
pub enum IntcodeError {
    UnknownOpcode { opcode: i32, index: usize },
    WriteToConstantProhibited,
    IndexOutOfBounds,
}

#[derive(Debug)]
pub struct HaltedInterpreter {
    code: Vec<i32>,
    output: Vec<i32>,
}

impl HaltedInterpreter {
    fn new(code: Vec<i32>, output: Vec<i32>) -> Self {
        Self { code, output }
    }

    pub fn get_code(&self) -> &Vec<i32> {
        &self.code
    }

    pub fn get_output(&self) -> &Vec<i32> {
        &self.output
    }
}

impl IntcodeInterpreter {
    pub fn set_input(&mut self, input: impl Iterator<Item = i32>) {
        self.input = input.collect();
    }

    pub fn run(mut self) -> Result<HaltedInterpreter, IntcodeError> {
        let mut input_iter = self.input.iter();
        let mut output = Vec::new();

        loop {
            let next_code = *self
                .code
                .get(self.current_position)
                .ok_or(IntcodeError::IndexOutOfBounds)?;
            let opcode: Opcode = next_code.into();

            let mut parameters =
                Parameter::from_opcode(&opcode, &self.code[self.current_position + 1..])?;

            macro_rules! process_params {
                ([$($var: ident),+] => $action: block) => {
                    $(let $var = parameters.next().unwrap();)+

                    drop(parameters);

                    $action;
                };
            }

            match opcode.operation {
                Operation::Add => {
                    process_params!([a, b, target] => {
                        let a = a.read(&self.code)?;
                        let b = b.read(&self.code)?;

                        target.write(&mut self.code, a + b)?;
                    });
                }
                Operation::Multiply => {
                    process_params!([a, b, target] => {
                        let a = a.read(&self.code)?;
                        let b = b.read(&self.code)?;

                        target.write(&mut self.code, a * b)?;
                    });
                }
                Operation::Input => {
                    process_params!([target] => {
                        target.write(&mut self.code, *input_iter.next().ok_or(IntcodeError::IndexOutOfBounds)?)?;
                    });
                }
                Operation::Output => {
                    process_params!([source] => {
                        output.push(source.read(&self.code)?);
                    });
                }
                Operation::JumpIfTrue => {
                    process_params!([a, target] => {
                        if a.read(&self.code)? != 0 {
                            self.current_position = target.read(&self.code)? as usize;

                            continue;
                        }
                    });
                }
                Operation::JumpIfFalse => {
                    process_params!([a, target] => {
                        if a.read(&self.code)? == 0 {
                            self.current_position = target.read(&self.code)? as usize;

                            continue;
                        }
                    });
                }
                Operation::LessThan => {
                    process_params!([a, b, target] => {
                        if a.read(&self.code)? < b.read(&self.code)? {
                            target.write(&mut self.code, 1)?;
                        } else {
                            target.write(&mut self.code, 0)?;
                        }
                    });
                }
                Operation::Equals => {
                    process_params!([a, b, target] => {
                        if a.read(&self.code)? == b.read(&self.code)? {
                            target.write(&mut self.code, 1)?;
                        } else {
                            target.write(&mut self.code, 0)?;
                        }
                    });
                }
                Operation::End => {
                    drop(parameters);

                    break Ok(HaltedInterpreter::new(self.code, output));
                }
                Operation::Unknown => {
                    break Err(IntcodeError::UnknownOpcode {
                        opcode: next_code,
                        index: self.current_position,
                    })
                }
            }

            self.current_position += opcode.parameters_count() + 1;
        }
    }
}

impl From<Vec<i32>> for IntcodeInterpreter {
    fn from(code: Vec<i32>) -> Self {
        Self {
            input: Vec::new(),
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
                let interpreter: IntcodeInterpreter = $input.to_vec().into();
                let halted = interpreter.run().unwrap();

                let result = halted.get_code();
                assert_eq!(result[..], $expected[..]);
            };
        }

        // day 2
        test_intcode!([1, 0, 0, 0, 99], [2, 0, 0, 0, 99]);

        test_intcode!([2, 3, 0, 3, 99], [2, 3, 0, 6, 99]);

        test_intcode!([2, 4, 4, 5, 99, 0], [2, 4, 4, 5, 99, 9801]);

        test_intcode!([1, 1, 1, 4, 99, 5, 6, 0, 99], [30, 1, 1, 4, 2, 5, 6, 0, 99]);

        // day 5
        test_intcode!([1002, 4, 3, 4, 33], [1002, 4, 3, 4, 99]);

        test_intcode!([1101, 100, -1, 4, 0], [1101, 100, -1, 4, 99]);
    }

    #[test]
    fn test_intcode_io_examples() {
        macro_rules! test_io {
            ($input: expr => $code: expr, $expected: expr) => {
                let mut interpreter: IntcodeInterpreter = $code.to_vec().into();
                interpreter.set_input($input.into_iter().copied());
                let halted = interpreter.run().unwrap();

                let result = halted.get_output();
                assert_eq!(result[..], $expected[..]);
            };
        }

        // input equals 8 (positional)
        test_io!([1] => [3, 9, 8, 9, 10, 9, 4, 9, 99,-1, 8], [0]);
        test_io!([8] => [3, 9, 8, 9, 10, 9, 4, 9, 99,-1, 8], [1]);
        test_io!([9] => [3, 9, 8, 9, 10, 9, 4, 9, 99,-1, 8], [0]);

        // input less than 8 (positional)
        test_io!([1] => [3, 9, 7, 9, 10, 9, 4, 9, 99,-1, 8], [1]);
        test_io!([5] => [3, 9, 7, 9, 10, 9, 4, 9, 99,-1, 8], [1]);
        test_io!([8] => [3, 9, 7, 9, 10, 9, 4, 9, 99,-1, 8], [0]);
        test_io!([9] => [3, 9, 7, 9, 10, 9, 4, 9, 99,-1, 8], [0]);

        // input equals 8 (immediate)
        test_io!([1] => [3, 3, 1108,-1, 8, 3, 4, 3, 99], [0]);
        test_io!([8] => [3, 3, 1108,-1, 8, 3, 4, 3, 99], [1]);
        test_io!([9] => [3, 3, 1108,-1, 8, 3, 4, 3, 99], [0]);

        // input less than 8 (positional)
        test_io!([1] => [3, 9, 7, 9, 10, 9, 4, 9, 99,-1, 8], [1]);
        test_io!([5] => [3, 9, 7, 9, 10, 9, 4, 9, 99,-1, 8], [1]);
        test_io!([8] => [3, 9, 7, 9, 10, 9, 4, 9, 99,-1, 8], [0]);
        test_io!([9] => [3, 9, 7, 9, 10, 9, 4, 9, 99,-1, 8], [0]);

        // input non-zero (positional)
        test_io!([0] => [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99,-1, 0, 1, 9], [0]);
        test_io!([1] => [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99,-1, 0, 1, 9], [1]);
        test_io!([-1] => [3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99,-1, 0, 1, 9], [1]);

        // input non-zero (immediate)
        test_io!([0] => [3, 3, 1105,-1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], [0]);
        test_io!([1] => [3, 3, 1105,-1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], [1]);
        test_io!([-1] => [3, 3, 1105,-1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], [1]);

        // cmp input to 8 (999 — less, 1000 — eq, 1001 — greater)
        test_io!([1] => [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
            1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
            999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
        ], [999]);
        test_io!([8] => [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
            1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
            999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
        ], [1000]);
        test_io!([81] => [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
            1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
            999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99,
        ], [1001]);
    }
}
