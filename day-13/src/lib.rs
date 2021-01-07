use std::{error::Error, path::PathBuf, str::FromStr};

use advent_utils::{read_file, Part, Solver};

use intcode::Interpreter;

mod arcade;
mod consts;
mod tile;

pub use arcade::{Emulator, EmulatorMode};

#[derive(Debug)]
pub struct Solution {
    interpreter: Interpreter,
    mode: Option<EmulatorMode>,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let interpreter: Interpreter = input_data.parse()?;

        Ok(Self {
            interpreter,
            mode: Some(EmulatorMode::Auto),
        })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        13
    }

    fn solve(&self, part: Part) -> String {
        let mut interpreter = self.interpreter.clone();

        match part {
            Part::One => {
                interpreter.run();

                let output = interpreter.get_output().expect("intcode program failed");
                let blocks_count = output.chunks_exact(3).filter(|chunk| chunk[2] == 2).count();

                format!("blocks on screen: {}", blocks_count)
            }
            Part::Two => {
                let mut code = interpreter.get_code().unwrap().to_vec();
                code[0] = 2;

                let interpreter = Interpreter::from(code);
                let emulator =
                    Emulator::new(interpreter, self.mode.expect("emulator mode not set"));

                let score = emulator.play().unwrap().unwrap();
                format!("Final score: {}", score)
            }
        }
    }
}

impl Solution {
    pub fn try_from_file_with_mode(
        input_file: PathBuf,
        mode: Option<EmulatorMode>,
    ) -> Result<Self, Box<dyn Error>> {
        read_file(input_file)?
            .parse()
            .map(|mut solution: Solution| {
                solution.mode = mode;

                solution
            })
    }
}
