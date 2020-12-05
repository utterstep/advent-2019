use std::{convert::TryFrom, error::Error, path::PathBuf};

use advent_utils::{read_file, Part, Solver};

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

impl TryFrom<PathBuf> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: PathBuf) -> Result<Self, Self::Error> {
        let interpreter: Interpreter = read_file(input_file)?.parse()?;

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
