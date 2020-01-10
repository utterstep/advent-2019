use std::error::Error;

use advent_utils::{get_config, read_file, Part};

mod orbital_system;

use orbital_system::System;

const N_STEPS: usize = 100_000_000;

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;
    let mut system = read_file(config.input_file)?
        .trim()
        .parse::<System>()
        .unwrap();

    match config.part {
        Part::One => {
            for _ in 0..N_STEPS {
                system.advance();
            }

            println!("Total energy after {} runs: {}", N_STEPS, system.energy());
        }
        Part::Two => todo!(),
    }

    Ok(())
}
