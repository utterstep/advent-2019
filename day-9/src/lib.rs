use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};

use intcode::Interpreter;

#[derive(Debug)]
pub struct Solution {
    interpreter: Interpreter,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let interpreter: Interpreter = input_data.parse()?;

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
