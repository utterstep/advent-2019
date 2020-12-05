use std::{convert::TryFrom, error::Error, path::PathBuf};

use advent_utils::{read_file, Part, Solver};

mod orbit_graph;

use orbit_graph::{Orbit, Planets};

#[derive(Debug)]
pub struct Solution {
    input: String,
}

impl TryFrom<PathBuf> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: PathBuf) -> Result<Self, Self::Error> {
        let input = read_file(input_file)?;

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
