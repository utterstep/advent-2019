use std::{error::Error, fs::File, io::Read};

use advent_utils::{get_config, Part};
use itertools::iproduct;

mod intcode;

use intcode::IntcodeInterpreter;

// FIXME: extract to Config
const TARGET: u32 = 19_690_720;

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;

    let mut code_str = String::new();
    File::open(config.input_file)?.read_to_string(&mut code_str)?;

    let mut code: Vec<u32> = code_str
        .trim()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    match config.part {
        Part::One => {
            code[1] = 12;
            code[2] = 2;

            let mut interpreter: IntcodeInterpreter = code.into();
            interpreter.run().unwrap();

            println!("position 0 value is: {}", interpreter.get_code()[0]);
        }
        Part::Two => {
            let res = iproduct!(0..100, 0..100).find_map(|(noun, verb)| {
                let mut code = code.clone();
                code[1] = noun;
                code[2] = verb;

                let mut interpreter: IntcodeInterpreter = code.into();
                match interpreter.run() {
                    Ok(_) => match interpreter.get_code()[0] {
                        TARGET => Some(noun * 100 + verb),
                        _ => None,
                    },
                    Err(_) => None,
                }
            });

            println!("noun and verb for target {} are: {}", TARGET, res.unwrap());
        }
    }

    Ok(())
}