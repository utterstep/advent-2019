use std::error::Error;

use advent_utils::{get_config, parse_file, Part};

mod orbit_graph;

use orbit_graph::Planets;

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;
    let orbits = parse_file(&config.input_file)?;
    let planets: Planets = orbits.into_iter().collect();

    match config.part {
        Part::One => println!("checksum is: {}", planets.orbit_count_checksums()),
        Part::Two => println!(
            "hops to Santa: {}",
            planets.steps_to_lca("YOU", "SAN").unwrap()
        ),
    }

    Ok(())
}
