use std::error::Error;

use advent_utils::{get_custom_config, Config as BaseConfig, Solver};
use serde::Deserialize;

use day_22::{SimulationMode, Solution};

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(flatten)]
    base: BaseConfig,
    mode: SimulationMode,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = get_custom_config()?;

    let solution = Solution::try_from_file_with_mode(config.base.input_file, config.mode)?;

    println!("{}", solution.solve(config.base.part));

    Ok(())
}
