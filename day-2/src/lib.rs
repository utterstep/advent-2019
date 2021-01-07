use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};
use intcode::Interpreter;
use itertools::iproduct;

const TARGET: i64 = 19_690_720;

#[derive(Debug)]
pub struct Solution {
    code: Vec<i64>,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let code: Vec<_> = input_data
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
