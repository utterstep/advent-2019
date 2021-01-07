use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};

mod orbital_system;
mod utils;

use orbital_system::System;

const N_STEPS: usize = 1000;

#[derive(Debug)]
pub struct Solution {
    system: System,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let system = input_data.trim().parse::<System>()?;

        Ok(Self { system })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        12
    }

    fn solve(&self, part: Part) -> String {
        let mut system = self.system.clone();

        match part {
            Part::One => {
                for _ in 0..N_STEPS {
                    system.advance();
                }

                format!("Total energy after {} runs: {}", N_STEPS, system.energy())
            }
            Part::Two => format!("System cycle length is: {}", system.cycle_length()),
        }
    }
}
