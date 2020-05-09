use std::collections::BTreeSet;

use advent_utils::{get_config, read_file, Part};
use intcode::Interpreter;

mod network;

use network::{Network, NAT};

const NETWORK_SIZE: usize = 50;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config()?;
    let interpreter: Interpreter = read_file(config.input_file)?.parse()?;
    let mut network = Network::new(interpreter, NETWORK_SIZE);

    match config.part {
        Part::One => {
            let transmission = network
                .find(|p| p.dst() == NAT)
                .expect("suitable transmission not found");

            #[cfg(debug_assertions)]
            println!("processed {} opcodes", network.get_processed_opcodes());

            println!("target: {:?}", transmission);
        }
        Part::Two => {
            let mut ys = BTreeSet::new();

            let transmission = network
                .find(|t| t.src() == NAT && !ys.insert(t.payload().y()))
                .expect("suitable transmission not found");

            #[cfg(debug_assertions)]
            println!("processed {} opcodes", network.get_processed_opcodes());

            println!("target: {:?}", transmission);
        }
    }

    Ok(())
}
