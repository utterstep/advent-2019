use std::{borrow::Borrow, convert::TryInto};

use crate::opcode::{Opcode, Operation, ParameterMode};

#[derive(Debug)]
struct Parameter {
    mode: ParameterMode,
    value: i64,
}

#[derive(Debug, Copy, Clone)]
pub enum IntcodeVmError {
    UnknownOpcode { opcode: i64, index: usize },
    WriteToConstantProhibited,
    NegativeIndex { index: i64 },
    PartialOpcode,
    InsufficientInputData,
}

#[derive(Debug, Clone)]
pub struct IntcodeVM {
    code: Vec<i64>,
    current_position: usize,
    relative_base: i64,
    #[cfg(debug_assertions)]
    pub(crate) processed_opcode_counter: usize,
}

#[derive(Debug)]
pub(crate) enum IntcodeVmStopCause {
    Halted,
    WaitingForInput,
}

impl From<Vec<i64>> for IntcodeVM {
    fn from(code: Vec<i64>) -> Self {
        IntcodeVM {
            code,
            current_position: 0,
            relative_base: 0,
            #[cfg(debug_assertions)]
            processed_opcode_counter: 0,
        }
    }
}

impl IntcodeVM {
    pub(crate) fn get_code(&self) -> &[i64] {
        &self.code
    }

    pub(crate) fn run_with_io(
        &mut self,
        input: impl IntoIterator<Item = impl Borrow<i64>>,
        output: &mut Vec<i64>,
    ) -> Result<IntcodeVmStopCause, IntcodeVmError> {
        let mut input = input.into_iter();

        loop {
            let next_code = *self.code.get(self.current_position).unwrap_or(&0);
            let opcode: Opcode = next_code.into();

            #[cfg(debug_assertions)]
            {
                self.processed_opcode_counter += 1;
            }

            let mut parameters = self.get_params(&opcode)?;

            macro_rules! process_params {
                ($($var: ident),+ => $action: block) => {
                    $(let $var = parameters.next().unwrap();)+

                    drop(parameters);

                    $action;
                };
            }

            match opcode.operation {
                Operation::Add => {
                    process_params!(a, b, target => {
                        let a = self.read(a)?;
                        let b = self.read(b)?;

                        self.write(target, a + b)?;
                    });
                }
                Operation::Multiply => {
                    process_params!(a, b, target => {
                        let a = self.read(a)?;
                        let b = self.read(b)?;

                        self.write(target, a * b)?;
                    });
                }
                Operation::Input => {
                    process_params!(target => {
                        match input.next() {
                            Some(value) => self.write(target, *value.borrow())?,
                            None => return Ok(IntcodeVmStopCause::WaitingForInput),
                        }
                    });
                }
                Operation::Output => {
                    process_params!(source => {
                        output.push(self.read(source)?);
                    });
                }
                Operation::JumpIfTrue => {
                    process_params!(a, target => {
                        if self.read(a)? != 0 {
                            self.current_position = self.read(target)? as usize;

                            continue;
                        }
                    });
                }
                Operation::JumpIfFalse => {
                    process_params!(a, target => {
                        if self.read(a)? == 0 {
                            self.current_position = self.read(target)? as usize;

                            continue;
                        }
                    });
                }
                Operation::LessThan => {
                    process_params!(a, b, target => {
                        if self.read(a)? < self.read(b)? {
                            self.write(target, 1)?;
                        } else {
                            self.write(target, 0)?;
                        }
                    });
                }
                Operation::Equals => {
                    process_params!(a, b, target => {
                        if self.read(a)? == self.read(b)? {
                            self.write(target, 1)?;
                        } else {
                            self.write(target, 0)?;
                        }
                    });
                }
                Operation::AdjRelBase => {
                    process_params!(value => {
                        self.relative_base += self.read(value)?;
                    });
                }
                Operation::End => {
                    drop(parameters);

                    break Ok(IntcodeVmStopCause::Halted);
                }
                Operation::Unknown => {
                    break Err(IntcodeVmError::UnknownOpcode {
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
    ) -> Result<impl Iterator<Item = Parameter> + 'a, IntcodeVmError> {
        let parameters_count = opcode.parameters_count();
        let code = &self.code[self.current_position + 1..];

        Ok(code
            .get(..parameters_count)
            .ok_or(IntcodeVmError::PartialOpcode)?
            .iter()
            .zip(opcode.parameter_modes.iter())
            .map(|(&value, &mode)| Parameter { value, mode }))
    }

    #[inline]
    fn read(&mut self, param: Parameter) -> Result<i64, IntcodeVmError> {
        macro_rules! read_default {
            ($idx: expr) => {{
                let idx: usize = $idx
                    .try_into()
                    .map_err(|_| IntcodeVmError::NegativeIndex { index: $idx })?;

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
    fn write(&mut self, param: Parameter, value: i64) -> Result<(), IntcodeVmError> {
        macro_rules! write_with_resize {
            ($idx: expr) => {{
                let idx: usize = $idx
                    .try_into()
                    .map_err(|_| IntcodeVmError::NegativeIndex { index: param.value })?;

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
            ParameterMode::Immediate => Err(IntcodeVmError::WriteToConstantProhibited),
            ParameterMode::Position => write_with_resize!(param.value),
            ParameterMode::Relative => write_with_resize!(self.relative_base + param.value),
        }
    }
}
