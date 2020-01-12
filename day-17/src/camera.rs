use std::{convert::TryFrom, num::TryFromIntError};

use intcode::{IntcodeVmError, Interpreter};

#[derive(Debug)]
pub struct Camera {
    interpreter: Interpreter,
}

impl From<Interpreter> for Camera {
    fn from(interpreter: Interpreter) -> Self {
        Self { interpreter }
    }
}

#[derive(Debug)]
pub enum ViewError {
    IntcodeError(IntcodeVmError),
    TryFromIntError,
}

impl From<IntcodeVmError> for ViewError {
    fn from(err: IntcodeVmError) -> Self {
        Self::IntcodeError(err)
    }
}

impl From<TryFromIntError> for ViewError {
    fn from(_err: TryFromIntError) -> Self {
        Self::TryFromIntError
    }
}

impl Camera {
    pub fn get_view(&mut self) -> Result<String, ViewError> {
        self.interpreter.run();

        self.interpreter
            .get_output()?
            .iter()
            .map(|&v| u8::try_from(v).map(char::from).map_err(Into::into))
            .collect()
    }
}
