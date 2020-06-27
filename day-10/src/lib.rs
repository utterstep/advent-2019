use std::{convert::TryFrom, error::Error};

use advent_utils::{read_file, Part, Solver};

mod rational;
mod space;

use space::Space;

#[derive(Debug)]
pub struct Solution {
    space: Space,
}

impl TryFrom<String> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: String) -> Result<Self, Self::Error> {
        let space = Space::from(read_file(input_file)?.as_str());

        Ok(Self { space })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        10
    }

    fn solve(&self, part: Part) -> String {
        let (station, num_seen) = self.space.best_station_location().unwrap();

        match part {
            Part::One => format!(
                "station best location is on {:?}, will see {} asteroids at once",
                station, num_seen
            ),
            Part::Two => format!(
                "asteroid to be destroyed 200th: {:?}",
                self.space.asteroids_in_vaporize_order(station).nth(199)
            ),
        }
    }
}
