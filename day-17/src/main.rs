use std::error::Error;

use advent_utils::{get_config, read_file, Part};

use intcode::Interpreter;

mod compression;
mod robot;
mod space;

use robot::Robot;
use space::Space;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;
    let code_str = read_file(config.input_file)?;

    let mut code: Vec<_> = code_str
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()?;

    let interpreter = Interpreter::from(code.clone());
    let mut robot = Robot::from(interpreter);
    let view = robot.get_view().unwrap();
    let space: Space = view.trim().parse().unwrap();

    match config.part {
        Part::One => {
            println!(
                "sum of the alignment parameters is: {}",
                space.scaffold_alignment_parameters().sum::<usize>()
            );
        }
        Part::Two => {
            let commands = space.traverse().collect::<Vec<_>>();
            let (dict, compressed) =
                compression::compress(&commands, 20).expect("failed to compress");

            code[0] = 2;
            let interpreter = Interpreter::from(code);
            let mut robot = Robot::from(interpreter);

            println!(
                "sum of cleaned dust: {}",
                robot
                    .run_cleaning(dict, compressed)
                    .expect("cleaning failed")
            );
        }
    };

    Ok(())
}
