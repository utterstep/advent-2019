use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};

use intcode::Interpreter;

mod robot;
mod solver;
mod utils;

use robot::Color;
use solver::paint_panels;

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
        11
    }

    fn solve(&self, part: Part) -> String {
        let interpreter = self.interpreter.clone();

        match part {
            Part::One => {
                let robot = paint_panels(interpreter, Color::Black).unwrap();

                format!("{} panels will be painted", robot.painted_panels_count())
            }
            Part::Two => {
                let robot = paint_panels(interpreter, Color::White).unwrap();

                format!(
                    "robot painted following:\n{}",
                    robot.painted_panels_display()
                )
            }
        }
    }
}
