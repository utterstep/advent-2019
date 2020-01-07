use std::error::Error;

use advent_utils::{get_custom_config, read_file, Part};
use itertools::iproduct;
use serde::Deserialize;

use intcode::Interpreter;

#[derive(Debug, Deserialize)]
struct Config {
    input_file: String,
    part: Part,
    target: Option<i64>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = get_custom_config()?;

    let mut code: Vec<_> = read_file(config.input_file)?
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()?;

    match config.part {
        Part::One => {
            code[1] = 12;
            code[2] = 2;

            let mut interpreter: Interpreter = code.into();
            interpreter.run();

            println!(
                "position 0 value is: {}",
                interpreter.get_code().unwrap()[0]
            );
        }
        Part::Two => {
            let target = config.target.expect("unspecified target for part two");

            let res = iproduct!(0..100, 0..100).find_map(|(noun, verb)| {
                let mut code = code.clone();
                code[1] = noun;
                code[2] = verb;

                let mut interpreter: Interpreter = code.into();
                interpreter.run();

                match interpreter.get_code() {
                    Ok(code) => match code[0] {
                        n if n == target => Some(noun * 100 + verb),
                        _ => None,
                    },
                    Err(_) => None,
                }
            });

            println!("noun and verb for target {} are: {}", target, res.unwrap());
        }
    }

    Ok(())
}
