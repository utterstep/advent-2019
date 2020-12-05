use std::{error::Error, path::PathBuf};

use advent_utils::{get_custom_config, Part, Solver};
use serde::Deserialize;

use day_13::{EmulatorMode, Solution};

#[derive(Debug, Deserialize)]
struct Config {
    input_file: PathBuf,
    part: Part,
    mode: Option<EmulatorMode>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = get_custom_config()?;

    let solution = Solution::try_from_file_with_mode(config.input_file, config.mode)?;

    println!("{}", solution.solve(config.part));

    Ok(())
}
