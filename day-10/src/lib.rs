use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};

mod rational;
mod space;

use space::Space;

#[derive(Debug)]
pub struct Solution {
    space: Space,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let space = input_data.into();

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
