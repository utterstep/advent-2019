use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};
use itertools::iproduct;

const VIEW_DISTANCE: i64 = 50;

mod solve;

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
        19
    }

    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => {
                let affected_points = iproduct!(0..VIEW_DISTANCE, 0..VIEW_DISTANCE)
                    // iproduct actually produces order (y, x),
                    // but is doesn't matter in case area is symmetric (like this one, 50x50)
                    .map(|point| solve::check_point(&self.code, point))
                    .filter(|&output| output)
                    .count();

                format!("points are affected by tractor ray: {}", affected_points)
            }
            Part::Two => {
                let (x, y) = solve::find_square_base(&self.code, 100);
                format!("square base is at: {}", x * 10000 + y)
            }
        }
    }
}
