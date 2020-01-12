use std::error::Error;

use advent_utils::{get_config, read_file, Part};

use intcode::Interpreter;

mod camera;
mod space;

use camera::Camera;
use space::Space;

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;

    let interpreter: Interpreter = read_file(config.input_file)?.parse()?;
    let mut camera = Camera::from(interpreter);
    let view = camera.get_view().unwrap();
    let space: Space = view.trim().parse().unwrap();

    match config.part {
        Part::One => {
            println!(
                "sum of the alignment parameters is: {}",
                space.scaffold_alignment_parameters().sum::<usize>()
            );
        }
        Part::Two => todo!(),
    };

    Ok(())
}
