use std::{error::Error, fs::File, io::Read};

use advent_utils::{get_config, Part};

use intcode::Interpreter;

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;

    let mut code_str = String::new();
    File::open(config.input_file)?.read_to_string(&mut code_str)?;

    let code: Vec<_> = code_str
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()?;

    let mut interpreter: Interpreter = code.into();

    match config.part {
        Part::One => {
            interpreter.run_with_input(&[1]);

            println!(
                "diagnostics output is: {:?}",
                interpreter.get_output().unwrap()
            );
        }
        Part::Two => {
            interpreter.run_with_input(&[5]);

            println!(
                "diagnostics output is: {:?}",
                interpreter.get_output().unwrap()
            );
        }
    }

    Ok(())
}
