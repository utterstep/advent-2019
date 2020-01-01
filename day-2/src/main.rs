use std::{error::Error, fs::File, io::Read};

use advent_utils::{get_custom_config, Part};
use itertools::iproduct;
use serde::Deserialize;

use intcode::IntcodeInterpreter;

#[derive(Debug, Deserialize)]
struct Config {
    input_file: String,
    part: Part,
    target: Option<i64>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = get_custom_config()?;

    let mut code_str = String::new();
    File::open(config.input_file)?.read_to_string(&mut code_str)?;

    let mut code: Vec<_> = code_str
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()?;

    match config.part {
        Part::One => {
            code[1] = 12;
            code[2] = 2;

            let interpreter: IntcodeInterpreter = code.into();
            let halted = interpreter.run().unwrap();

            println!("position 0 value is: {}", halted.get_code()[0]);
        }
        Part::Two => {
            let target = config.target.expect("unspecified target for part two");

            let res = iproduct!(0..100, 0..100).find_map(|(noun, verb)| {
                let mut code = code.clone();
                code[1] = noun;
                code[2] = verb;

                let interpreter: IntcodeInterpreter = code.into();
                match interpreter.run() {
                    Ok(halted) => match halted.get_code()[0] {
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
