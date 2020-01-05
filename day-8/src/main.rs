use std::error::Error;

use advent_utils::{get_config, read_file, Part};

mod sif;

use sif::SpaceImage;

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;
    let pixels: Vec<_> = read_file(config.input_file)?
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let image = SpaceImage::from_slice(25, 6, &pixels);

    match config.part {
        Part::One => println!("image checksum is: {}", image.checksum().unwrap()),
        Part::Two => println!("composed image looks like:\n{}", image.compose()),
    }

    Ok(())
}
