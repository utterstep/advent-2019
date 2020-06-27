use std::{convert::TryFrom, error::Error};

use advent_utils::{read_file, Part, Solver};
use intcode::Interpreter;
use itertools::iproduct;

const TARGET: i64 = 19_690_720;

#[derive(Debug)]
pub struct Solution {
    code: Vec<i64>,
}

impl TryFrom<String> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: String) -> Result<Self, Self::Error> {
        let code: Vec<_> = read_file(input_file)?
            .trim()
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self { code })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        2
    }

    fn solve(&self, part: Part) -> String {
        let mut code = self.code.clone();

        match part {
            Part::One => {
                code[1] = 12;
                code[2] = 2;

                let mut interpreter: Interpreter = code.into();
                interpreter.run();

                format!(
                    "position 0 value is: {}",
                    interpreter.get_code().unwrap()[0]
                )
            }
            Part::Two => {
                let res = iproduct!(0..100, 0..100).find_map(|(noun, verb)| {
                    let mut code = code.clone();
                    code[1] = noun;
                    code[2] = verb;

                    let mut interpreter: Interpreter = code.into();
                    interpreter.run();

                    match interpreter.get_code() {
                        Ok(code) => match code[0] {
                            n if n == TARGET => Some(noun * 100 + verb),
                            _ => None,
                        },
                        Err(_) => None,
                    }
                });

                format!("noun and verb for target {} are: {}", TARGET, res.unwrap())
            }
        }
    }
}
