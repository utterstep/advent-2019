use std::{convert::TryFrom, error::Error, str::FromStr};

use advent_utils::{Part, Solver};

mod orbit_graph;

use orbit_graph::{Orbit, Planets};

#[derive(Debug)]
pub struct Solution {
    input: String,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let input = input_data.to_owned();

        Ok(Self { input })
    }
}

impl<'a> Solver for Solution {
    fn day_number() -> u32 {
        6
    }

    fn solve(&self, part: Part) -> String {
        let orbits = self
            .input
            .split_terminator('\n')
            .map(|s| Orbit::try_from(s).unwrap());
        let planets: Planets = orbits.collect();

        match part {
            Part::One => format!("checksum is: {}", planets.orbit_count_checksums()),
            Part::Two => format!(
                "hops to Santa: {}",
                planets.steps_to_lca("YOU", "SAN").unwrap()
            ),
        }
    }
}
