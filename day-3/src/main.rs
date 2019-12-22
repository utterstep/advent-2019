use std::error::Error;

use advent_utils::{get_config, parse_file, Part};

mod direction;
mod point;
mod segment;
mod wire;

use wire::Wire;

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;
    let wires: Vec<Wire> = parse_file(config.input_file)?;

    match config.part {
        Part::One => {
            let w1 = &wires[0];
            let w2 = &wires[1];

            println!(
                "distance from closest intersection to 0: {}",
                w1.intersections_with(w2).map(|p| p.manhattan_to_zero()).min().unwrap(),
            );
        },
        Part::Two => {
            let w1 = &wires[0];
            let w2 = &wires[1];

            println!(
                "steps to closest intersection: {}",
                w1.steps_to_intersections_with(&w2).min_by_key(|(steps, _)| *steps).unwrap().0,
            )
        }
    }

    Ok(())
}
