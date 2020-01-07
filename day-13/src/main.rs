use std::error::Error;

use advent_utils::{get_config, read_file, Part};

use intcode::Interpreter;

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;

    let mut interpreter: Interpreter = read_file(config.input_file)?.parse()?;
    interpreter.run();

    match config.part {
        Part::One => {
            let output = interpreter.get_output().expect("intcode program failed");
            let blocks_count = output.chunks_exact(3).filter(|chunk| chunk[2] == 2).count();

            println!("blocks on screen: {}", blocks_count);
        }
        Part::Two => todo!(),
    }

    Ok(())
}
