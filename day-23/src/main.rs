use advent_utils::{get_config, read_file, Part};
use intcode::Interpreter;

mod network;

use network::Network;

const NETWORK_SIZE: usize = 50;
const DST_TO_FIND: i64 = 255;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config()?;
    let interpreter: Interpreter = read_file(config.input_file)?.parse()?;
    let mut network = Network::new(interpreter, NETWORK_SIZE);

    match config.part {
        Part::One => {
            let packet = network
                .find(|p| p.dst() == DST_TO_FIND)
                .expect("packet not found");

            #[cfg(debug_assertions)]
            println!("processed {} opcodes", network.get_processed_opcodes());

            println!("target packet: {:?}", packet);
        }
        Part::Two => todo!(),
    }

    Ok(())
}
