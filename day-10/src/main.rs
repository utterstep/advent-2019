use std::error::Error;

use advent_utils::{get_config, read_file, Part};

mod rational;
mod space;

use space::Space;

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;
    let space = Space::from(read_file(config.input_file)?.as_str());
    let (station, num_seen) = space.best_station_location().unwrap();

    match config.part {
        Part::One => println!(
            "station best location is on {:?}, will see {} asteroids at once",
            station, num_seen
        ),
        Part::Two => println!(
            "asteroid to be destroyed 200th: {:?}",
            space.asteroids_in_vaporize_order(station).nth(199)
        ),
    }

    Ok(())
}
