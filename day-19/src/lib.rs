use std::{convert::TryFrom, error::Error};

use advent_utils::{read_file, Part, Solver};
use itertools::iproduct;

const VIEW_DISTANCE: i64 = 50;

mod solve;

#[derive(Debug)]
pub struct Solution {
    code: Vec<i64>,
}

impl TryFrom<String> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: String) -> Result<Self, Self::Error> {
        let code_str = read_file(input_file)?;

        let code: Vec<_> = code_str
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
