use std::{convert::TryFrom, error::Error};

use advent_utils::{read_file, Part, Solver};

use intcode::Interpreter;

pub struct Solution {
    interpreter: Interpreter,
}

impl TryFrom<String> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: String) -> Result<Self, Self::Error> {
        let interpreter: Interpreter = read_file(input_file)?.parse()?;

        Ok(Self { interpreter })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        9
    }

    fn solve(&self, part: Part) -> String {
        let mut interpreter = self.interpreter.clone();

        match part {
            Part::One => {
                interpreter.run_with_input(&[1]);

                format!(
                    "diagnostics output is: {:?}",
                    interpreter.get_output().unwrap()
                )
            }
            Part::Two => {
                interpreter.run_with_input(&[2]);

                format!(
                    "diagnostics output is: {:?}",
                    interpreter.get_output().unwrap()
                )
            }
        }
    }
}
