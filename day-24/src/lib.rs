use std::{collections::BTreeSet, error::Error, str::FromStr};

use advent_utils::{Part, Solver};

mod world;

use world::World;

#[derive(Debug)]
pub struct Solution {
    world: World,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let world: World = input_data.parse()?;

        Ok(Self { world })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        24
    }

    fn solve(&self, part: Part) -> String {
        let mut world = self.world;

        match part {
            Part::One => {
                let mut existing = BTreeSet::new();

                while existing.insert(world) {
                    world = world.step();
                }

                format!(
                    "first world that repeats has biodiversity rating of: {}",
                    world.biodiversity()
                )
            }
            Part::Two => todo!(),
        }
    }

    fn implemented_parts() -> Vec<Part> {
        vec![Part::One]
    }
}
