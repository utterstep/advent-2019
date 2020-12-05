use std::{convert::TryFrom, error::Error, path::PathBuf};

use advent_utils::{read_file, Part, Solver};

mod amplifier;

#[derive(Debug)]
pub struct Solution {
    code: Vec<i64>,
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

        Ok(Self { code })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        7
    }

    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => {
                const SIMPLE_SETTINGS: [i64; 5] = [0, 1, 2, 3, 4];

                format!(
                    "max thruster power is: {}",
                    amplifier::find_max_power(&self.code, SIMPLE_SETTINGS).unwrap(),
                )
            }
            Part::Two => {
                const LOOP_SETTINGS: [i64; 5] = [5, 6, 7, 8, 9];

                format!(
                    "max thruster power is: {}",
                    amplifier::find_max_loop_power(&self.code, LOOP_SETTINGS).unwrap(),
                )
            }
        }
    }
}
