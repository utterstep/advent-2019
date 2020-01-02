use std::{error::Error, fs::File, io::Read};

use advent_utils::{get_config, Part};

use intcode::IntcodeInterpreter;

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;

    let mut code_str = String::new();
    File::open(config.input_file)?.read_to_string(&mut code_str)?;

    let code: Vec<_> = code_str
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()?;

    let interpreter: IntcodeInterpreter = code.into();

    match config.part {
        Part::One => {
            let halted = interpreter.run_with_input(&[1]).unwrap();

            println!("diagnostics output is: {:?}", halted.get_output());
        }
        Part::Two => {
            let halted = interpreter.run_with_input(&[5]).unwrap();

            println!("diagnostics output is: {:?}", halted.get_output());
        }
    }

    Ok(())
}
