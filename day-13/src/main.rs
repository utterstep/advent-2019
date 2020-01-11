use std::error::Error;

use serde::Deserialize;

use advent_utils::{get_custom_config, read_file, Part};

use intcode::Interpreter;

mod arcade;
mod consts;
mod tile;

use arcade::{Emulator, EmulatorMode};

#[derive(Debug, Deserialize)]
struct Config {
    input_file: String,
    part: Part,
    mode: Option<EmulatorMode>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = get_custom_config()?;

    let mut interpreter: Interpreter = read_file(config.input_file)?.parse()?;

    match config.part {
        Part::One => {
            interpreter.run();

            let output = interpreter.get_output().expect("intcode program failed");
            let blocks_count = output.chunks_exact(3).filter(|chunk| chunk[2] == 2).count();

            println!("blocks on screen: {}", blocks_count);
        }
        Part::Two => {
            let mut code = interpreter.get_code().unwrap().to_vec();
            code[0] = 2;

            let interpreter = Interpreter::from(code);
            let emulator = Emulator::new(interpreter, config.mode.expect("emulator mode not set"));

            let score = emulator.play().unwrap().unwrap();
            println!("Final score: {}", score);
        }
    }

    Ok(())
}
