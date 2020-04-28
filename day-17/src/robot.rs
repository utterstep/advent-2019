use std::{convert::TryFrom, num::TryFromIntError};

use intcode::{IntcodeVmError, Interpreter};

use crate::compression::CompressionDict;

#[derive(Debug)]
pub struct Robot {
    interpreter: Interpreter,
}

impl From<Interpreter> for Robot {
    fn from(interpreter: Interpreter) -> Self {
        Self { interpreter }
    }
}

#[derive(Debug)]
pub enum RobotError {
    IntcodeError(IntcodeVmError),
    TryFromIntError,
}

impl From<IntcodeVmError> for RobotError {
    fn from(err: IntcodeVmError) -> Self {
        Self::IntcodeError(err)
    }
}

impl From<TryFromIntError> for RobotError {
    fn from(_err: TryFromIntError) -> Self {
        Self::TryFromIntError
    }
}

impl Robot {
    pub fn get_view(&mut self) -> Result<String, RobotError> {
        self.interpreter.run();

        self.interpreter
            .get_output()?
            .iter()
            .map(|&v| u8::try_from(v).map(char::from).map_err(Into::into))
            .collect()
    }

    pub fn run_cleaning(
        &mut self,
        dict: CompressionDict,
        routine: String,
    ) -> Result<i64, RobotError> {
        #[cfg(debug_assertions)]
        const CAMERA_VIEW: &str = "y";
        #[cfg(not(debug_assertions))]
        const CAMERA_VIEW: &str = "n";

        let input = format!(
            "{main}\n{a}\n{b}\n{c}\n{view}\n",
            main = routine,
            a = &dict.a,
            b = &dict.b,
            c = &dict.c,
            view = CAMERA_VIEW,
        );

        self.interpreter
            .run_with_input(input.chars().map(|c| c as i64));

        self.interpreter.run();

        let output = self.interpreter.get_output()?;

        #[cfg(debug_assertions)]
        for &o in output {
            if o > 255 {
                return Ok(o);
            }
            print!("{}", char::from(u8::try_from(o)?));
        }

        Ok(*output.iter().last().unwrap())
    }
}
