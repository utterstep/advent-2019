use std::error::Error;

use advent_utils::{get_config, read_file, Part};

use intcode::Interpreter;

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;

    let mut interpreter: Interpreter = read_file(config.input_file)?.parse()?;

    match config.part {
        Part::One => {
            interpreter.run_with_input(&[1]);

            println!(
                "diagnostics output is: {:?}",
                interpreter.get_output().unwrap()
            );
        }
        Part::Two => {
            interpreter.run_with_input(&[2]);

            println!(
                "diagnostics output is: {:?}",
                interpreter.get_output().unwrap()
            );
        }
    }

    Ok(())
}
