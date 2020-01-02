use std::convert::TryInto;

mod opcode;

use opcode::{Opcode, Operation, ParameterMode};

#[derive(Debug)]
struct Parameter {
    mode: ParameterMode,
    value: i64,
}

#[derive(Debug)]
pub struct IntcodeInterpreter {
    input: Vec<i64>,
    code: Vec<i64>,
    current_position: usize,
    relative_base: i64,
}

#[derive(Debug)]
pub enum IntcodeError {
    UnknownOpcode { opcode: i64, index: usize },
    WriteToConstantProhibited,
    NegativeIndex { index: i64 },
    PartialOpcode,
    InsufficientInputData,
}

#[derive(Debug)]
pub struct HaltedInterpreter {
    code: Vec<i64>,
    output: Vec<i64>,
}

impl HaltedInterpreter {
    fn new(code: Vec<i64>, output: Vec<i64>) -> Self {
        Self { code, output }
    }

    pub fn get_code(&self) -> &Vec<i64> {
        &self.code
    }

    pub fn get_output(&self) -> &Vec<i64> {
        &self.output
    }
}

impl IntcodeInterpreter {
    pub fn run(self) -> Result<HaltedInterpreter, IntcodeError> {
        self.run_with_input(std::iter::empty())
    }

    pub fn run_with_input<'a>(
        mut self,
        input: impl IntoIterator<Item = &'a i64>,
    ) -> Result<HaltedInterpreter, IntcodeError> {
        let mut input = input.into_iter();
        let mut output = Vec::new();

        #[cfg(debug_assertions)]
        let mut opcode_counter = 0;

        loop {
            let next_code = *self.code.get(self.current_position).unwrap_or(&0);
            let opcode: Opcode = next_code.into();

            #[cfg(debug_assertions)]
            {
                opcode_counter += 1;
            }

            let mut parameters = self.get_params(&opcode)?;

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
                        let a = self.read(a)?;
                        let b = self.read(b)?;

                        self.write(target, a + b)?;
                    });
                }
                Operation::Multiply => {
                    process_params!([a, b, target] => {
                        let a = self.read(a)?;
                        let b = self.read(b)?;

                        self.write(target, a * b)?;
                    });
                }
                Operation::Input => {
                    process_params!([target] => {
                        self.write(target, *input.next().ok_or(IntcodeError::InsufficientInputData)?)?;
                    });
                }
                Operation::Output => {
                    process_params!([source] => {
                        output.push(self.read(source)?);
                    });
                }
                Operation::JumpIfTrue => {
                    process_params!([a, target] => {
                        if self.read(a)? != 0 {
                            self.current_position = self.read(target)? as usize;

                            continue;
                        }
                    });
                }
                Operation::JumpIfFalse => {
                    process_params!([a, target] => {
                        if self.read(a)? == 0 {
                            self.current_position = self.read(target)? as usize;

                            continue;
                        }
                    });
                }
                Operation::LessThan => {
                    process_params!([a, b, target] => {
                        if self.read(a)? < self.read(b)? {
                            self.write(target, 1)?;
                        } else {
                            self.write(target, 0)?;
                        }
                    });
                }
                Operation::Equals => {
                    process_params!([a, b, target] => {
                        if self.read(a)? == self.read(b)? {
                            self.write(target, 1)?;
                        } else {
                            self.write(target, 0)?;
                        }
                    });
                }
                Operation::AdjRelBase => {
                    process_params!([value] => {
                        self.relative_base += self.read(value)?;
                    });
                }
                Operation::End => {
                    drop(parameters);

                    #[cfg(debug_assertions)]
                    {
                        println!("{} opcodes processed", opcode_counter);
                    }

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

    /// Creates iterator over params from opcode and code slice, starting from current element
    ///
    /// Returns iterator instead of vec to avoid allocation (~3x performance penalty)
    #[inline]
    fn get_params<'a>(
        &'a self,
        opcode: &'a Opcode,
    ) -> Result<impl Iterator<Item = Parameter> + 'a, IntcodeError> {
        let parameters_count = opcode.parameters_count();
        let code = &self.code[self.current_position + 1..];

        Ok(code
            .get(..parameters_count)
            .ok_or(IntcodeError::PartialOpcode)?
            .iter()
            .zip(opcode.parameter_modes.iter())
            .map(|(&value, &mode)| Parameter { value, mode }))
    }

    #[inline]
    fn read(&mut self, param: Parameter) -> Result<i64, IntcodeError> {
        macro_rules! read_default {
            ($idx: expr) => {{
                let idx: usize = $idx
                    .try_into()
                    .map_err(|_| IntcodeError::NegativeIndex { index: $idx })?;

                Ok(*self.code.get(idx).unwrap_or(&0))
            }};
        }

        match param.mode {
            ParameterMode::Immediate => Ok(param.value),
            ParameterMode::Position => read_default!(param.value),
            ParameterMode::Relative => read_default!(self.relative_base + param.value),
        }
    }

    #[inline]
    fn write(&mut self, param: Parameter, value: i64) -> Result<(), IntcodeError> {
        macro_rules! write_with_resize {
            ($idx: expr) => {{
                let idx: usize = $idx
                    .try_into()
                    .map_err(|_| IntcodeError::NegativeIndex { index: param.value })?;

                if idx >= self.code.len() {
                    self.code.resize_with(idx + 1, Default::default);
                }

                unsafe {
                    *self.code.get_unchecked_mut(idx) = value;
                }

                Ok(())
            }};
        }

        match param.mode {
            ParameterMode::Immediate => Err(IntcodeError::WriteToConstantProhibited),
            ParameterMode::Position => write_with_resize!(param.value),
            ParameterMode::Relative => write_with_resize!(self.relative_base + param.value),
        }
    }
}

impl From<Vec<i64>> for IntcodeInterpreter {
    fn from(code: Vec<i64>) -> Self {
        Self {
            input: Vec::new(),
            code,
            current_position: 0,
            relative_base: 0,
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
                let interpreter: IntcodeInterpreter = $code.to_vec().into();
                let halted = interpreter.run_with_input(&$input).unwrap();

                let result = halted.get_output();
                assert_eq!(result[..], $expected[..]);
            };
        }

        // day-5

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

        // day-9

        // quine
        test_io!([] => [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99
        ], [109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]);

        // 16-digit
        test_io!([] => [1102, 34915192, 34915192, 7, 4, 7, 99, 0], [1219070632396864]);

        // identity
        test_io!([] => [104, 1125899906842624, 99], [1125899906842624]);
    }
}
