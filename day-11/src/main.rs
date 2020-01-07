use std::error::Error;

use advent_utils::{get_config, read_file, Part};

use intcode::Interpreter;

mod robot;
mod solver;
mod utils;

use robot::Color;
use solver::paint_panels;

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;

    let interpreter: Interpreter = read_file(config.input_file)?.parse()?;

    match config.part {
        Part::One => {
            let robot = paint_panels(interpreter, Color::Black).unwrap();

            println!("{} panels will be painted", robot.painted_panels_count());
        }
        Part::Two => {
            let robot = paint_panels(interpreter, Color::White).unwrap();

            println!(
                "robot painted following:\n{}",
                robot.painted_panels_display()
            );
        }
    }

    Ok(())
}
