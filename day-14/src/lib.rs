use std::{convert::TryFrom, error::Error};

use advent_utils::{read_file, Part, Solver};

mod precalc;
mod reactions;

use precalc::SolutionPrecalc;

#[derive(Debug)]
pub struct Solution {
    recipes: String,
}

impl<'a> TryFrom<String> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: String) -> Result<Self, Self::Error> {
        let recipes = read_file(input_file)?;

        Ok(Self { recipes })
    }
}

const ORE_QUANTITY: u64 = 1_000_000_000_000;

impl<'a> Solver for Solution {
    fn day_number() -> u32 {
        14
    }

    fn solve(&self, part: Part) -> String {
        let precalc = SolutionPrecalc::try_from(self.recipes.trim()).unwrap();

        match part {
            Part::One => format!(
                "You need {} ORE to produce one FUEL",
                precalc.ore_requirements(1)
            ),
            Part::Two => format!(
                "You can produce {} FUEL from {} ORE",
                precalc.available_fuel(ORE_QUANTITY),
                ORE_QUANTITY,
            ),
        }
    }
}
