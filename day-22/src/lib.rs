use std::{convert::TryFrom, error::Error};

use advent_utils::{parse_file, Part, Solver};
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
pub enum SimulationMode {
    Deck,
    Card,
}

pub struct Solution {
    movements: Vec<Movement>,
    mode: SimulationMode,
}

impl TryFrom<String> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: String) -> Result<Self, Self::Error> {
        let movements: Vec<Movement> = parse_file(input_file)?;

        Ok(Self {
            movements,
            mode: SimulationMode::Card,
        })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        22
    }

    fn solve(&self, part: Part) -> String {
        let card = match part {
            Part::One => CARD_TO_FIND_PT1,
            Part::Two => CARD_TO_FIND_PT2,
        };

        let deck_size = match part {
            Part::One => DECK_SIZE_PT1,
            Part::Two => DECK_SIZE_PT2,
        };

        let iters = match part {
            Part::One => ITERS_PT1,
            Part::Two => ITERS_PT2,
        };

        let mut simulator: Box<dyn Simulator> = match self.mode {
            SimulationMode::Card => Box::new(Card::new(deck_size as i64, card)),
            SimulationMode::Deck => Box::new(Deck::new(deck_size, card)),
        };

        for _ in 0..iters {
            for movement in &self.movements {
                simulator.execute(movement);
            }
        }

        format!("card {} is at idx {}", card, simulator.get_position())
    }

    fn implemented_parts() -> Vec<Part> {
        vec![Part::One]
    }
}

impl Solution {
    pub fn try_from_file_with_mode(
        input_file: String,
        mode: SimulationMode,
    ) -> Result<Self, Box<dyn Error>> {
        Self::try_from(input_file).map(|mut solution| {
            solution.mode = mode;

            solution
        })
    }
}
