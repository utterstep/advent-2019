use std::{convert::TryFrom, error::Error};

use advent_utils::{get_config, read_file, Part};

mod orbit_graph;

use orbit_graph::{Orbit, Planets};

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;
    let input = read_file(&config.input_file)?;
    let orbits = input
        .split_terminator('\n')
        .map(|s| Orbit::try_from(s).unwrap());
    let planets: Planets = orbits.collect();

    match config.part {
        Part::One => println!("checksum is: {}", planets.orbit_count_checksums()),
        Part::Two => println!(
            "hops to Santa: {}",
            planets.steps_to_lca("YOU", "SAN").unwrap()
        ),
    }

    Ok(())
}
