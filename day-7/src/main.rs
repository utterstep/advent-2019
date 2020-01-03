use std::{error::Error, fs::File, io::Read};

use advent_utils::{get_config, Part};

mod amplifier;

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;

    let mut code_str = String::new();
    File::open(config.input_file)?.read_to_string(&mut code_str)?;

    let code: Vec<_> = code_str
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()?;

    let possible_settings = [0, 1, 2, 3, 4];

    match config.part {
        Part::One => {
            println!(
                "max thruster power is: {}",
                amplifier::find_max_power(code, possible_settings).unwrap(),
            );
        },
        Part::Two => todo!(),
    };

    Ok(())
}
