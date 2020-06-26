use std::{collections::BTreeSet, convert::TryFrom, error::Error};

use advent_utils::{read_file, Part, Solver};
use intcode::Interpreter;

mod network;

use network::{Network, NAT};

const NETWORK_SIZE: usize = 50;

pub struct Solution {
    network: Network,
}

impl TryFrom<String> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: String) -> Result<Self, Self::Error> {
        let interpreter: Interpreter = read_file(input_file)?.parse()?;
        let network = Network::new(&interpreter, NETWORK_SIZE);

        Ok(Self { network })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        23
    }

    fn solve(&self, part: Part) -> String {
        let mut network = self.network.clone();

        match part {
            Part::One => {
                let transmission = network
                    .find(|p| p.dst() == NAT)
                    .expect("suitable transmission not found");

                #[cfg(debug_assertions)]
                println!("processed {} opcodes", network.get_processed_opcodes());

                format!("target: {:?}", transmission)
            }
            Part::Two => {
                let mut ys = BTreeSet::new();

                let transmission = network
                    .find(|t| t.src() == NAT && !ys.insert(t.payload().y()))
                    .expect("suitable transmission not found");

                #[cfg(debug_assertions)]
                println!("processed {} opcodes", network.get_processed_opcodes());

                format!("target: {:?}", transmission)
            }
        }
    }
}
