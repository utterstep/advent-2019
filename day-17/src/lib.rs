use std::{convert::TryFrom, error::Error, path::PathBuf};

use advent_utils::{read_file, Part, Solver};

use intcode::Interpreter;

mod compression;
mod robot;
mod space;

use robot::Robot;
use space::Space;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[derive(Debug)]
pub struct Solution {
    code: Vec<i64>,
    space: Space,
}

impl TryFrom<PathBuf> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: PathBuf) -> Result<Self, Self::Error> {
        let code_str = read_file(input_file)?;

        let code: Vec<_> = code_str
            .trim()
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        let interpreter = Interpreter::from(code.clone());
        let mut robot = Robot::from(interpreter);
        let view = robot.get_view()?;
        let space: Space = view.trim().parse()?;

        Ok(Self { code, space })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        17
    }

    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => format!(
                "sum of the alignment parameters is: {}",
                self.space.scaffold_alignment_parameters().sum::<usize>()
            ),
            Part::Two => {
                let commands = self.space.clone().traverse().collect::<Vec<_>>();
                let (dict, compressed) =
                    compression::compress(&commands, 20).expect("failed to compress");

                let mut code = self.code.clone();

                code[0] = 2;
                let interpreter = Interpreter::from(code);
                let mut robot = Robot::from(interpreter);

                format!(
                    "sum of cleaned dust: {}",
                    robot
                        .run_cleaning(&dict, &compressed)
                        .expect("cleaning failed")
                )
            }
        }
    }
}
