use advent_utils::{get_custom_config, parse_file, Config as BaseConfig, Part};
use serde::Deserialize;

mod card;
mod deck;
mod movement;
mod traits;

use card::Card;
use deck::Deck;
use movement::Movement;
use traits::Simulator;

const DECK_SIZE_PT1: usize = 10007;
const DECK_SIZE_PT2: usize = 119_315_717_514_047;
const CARD_TO_FIND_PT1: i64 = 2019;
const CARD_TO_FIND_PT2: i64 = 2020;

const ITERS_PT1: usize = 1;
const ITERS_PT2: usize = 101_741_582_076_661;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SimulationMode {
    Deck,
    Card,
}

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(flatten)]
    base: BaseConfig,
    mode: SimulationMode,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = get_custom_config()?;
    let movements: Vec<Movement> = parse_file(config.base.input_file)?;

    let card = match config.base.part {
        Part::One => CARD_TO_FIND_PT1,
        Part::Two => CARD_TO_FIND_PT2,
    };

    let deck_size = match config.base.part {
        Part::One => DECK_SIZE_PT1,
        Part::Two => DECK_SIZE_PT2,
    };

    let iters = match config.base.part {
        Part::One => ITERS_PT1,
        Part::Two => ITERS_PT2,
    };

    let mut simulator: Box<dyn Simulator> = match config.mode {
        SimulationMode::Card => Box::new(Card::new(deck_size as i64, card)),
        SimulationMode::Deck => Box::new(Deck::new(deck_size, card)),
    };

    for _ in 0..iters {
        for movement in &movements {
            simulator.execute(movement);
        }
    }

    println!("card {} is at idx {}", card, simulator.get_position());

    Ok(())
}
