use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};

mod amplifier;

#[derive(Debug)]
pub struct Solution {
    code: Vec<i64>,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let code: Vec<_> = input_data
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
