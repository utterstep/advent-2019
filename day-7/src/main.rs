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

    match config.part {
        Part::One => {
            const SIMPLE_SETTINGS: [i64; 5] = [0, 1, 2, 3, 4];

            println!(
                "max thruster power is: {}",
                amplifier::find_max_power(code, SIMPLE_SETTINGS).unwrap(),
            );
        }
        Part::Two => {
            const LOOP_SETTINGS: [i64; 5] = [5, 6, 7, 8, 9];

            println!(
                "max thruster power is: {}",
                amplifier::find_max_loop_power(code, LOOP_SETTINGS).unwrap(),
            );
        }
    };

    Ok(())
}
