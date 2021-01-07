use std::{error::Error, str::FromStr};

use advent_utils::{parse_raw_data, Part, Solver};

mod direction;
mod point;
mod segment;
mod wire;

use wire::Wire;

#[derive(Debug)]
pub struct Solution {
    w1: Wire,
    w2: Wire,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let mut wires: Vec<Wire> = parse_raw_data(input_data)?;
        let w2 = wires.pop().unwrap();
        let w1 = wires.pop().unwrap();

        Ok(Self { w1, w2 })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        3
    }

    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => format!(
                "distance from closest intersection to 0: {}",
                self.w1
                    .intersections_with(&self.w2)
                    .map(point::Point::manhattan_to_zero)
                    .min()
                    .unwrap(),
            ),
            Part::Two => format!(
                "steps to closest intersection: {}",
                self.w1
                    .steps_to_intersections_with(&self.w2)
                    .map(|(steps, _)| steps)
                    .min()
                    .unwrap(),
            ),
        }
    }
}
