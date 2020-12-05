use std::{convert::TryFrom, error::Error, path::PathBuf};

use advent_utils::{parse_file, Part, Solver};

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

impl TryFrom<PathBuf> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: PathBuf) -> Result<Self, Self::Error> {
        let mut wires: Vec<Wire> = parse_file(input_file)?;
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
