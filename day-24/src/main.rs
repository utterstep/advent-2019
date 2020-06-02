use advent_utils::{get_config, read_file, Part};

use std::{collections::BTreeSet, error::Error};

mod world;

use world::World;

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;
    let mut world: World = read_file(config.input_file)?.parse()?;

    match config.part {
        Part::One => {
            let mut existing = BTreeSet::new();

            while existing.insert(world) {
                world = world.step();
            }

            println!(
                "first world that repeats has biodiversity rating of: {}",
                world.biodiversity()
            );
        }
        Part::Two => todo!(),
    }

    Ok(())
}
