use std::{convert::TryFrom, error::Error};

use advent_utils::{read_file, Part, Solver};

mod sif;

use sif::SpaceImage;

#[derive(Debug)]
pub struct Solution {
    image: SpaceImage,
}

impl TryFrom<String> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: String) -> Result<Self, Self::Error> {
        let pixels: Vec<_> = read_file(input_file)?
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
