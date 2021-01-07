use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};

mod sif;

use sif::SpaceImage;

#[derive(Debug)]
pub struct Solution {
    image: SpaceImage,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let pixels: Vec<_> = input_data
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let image = SpaceImage::from_slice(25, 6, &pixels);

        Ok(Self { image })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        8
    }

    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => format!("image checksum is: {}", self.image.checksum().unwrap()),
            Part::Two => format!("composed image looks like:\n{}", self.image.compose()),
        }
    }
}
