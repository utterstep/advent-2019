use std::{convert::TryFrom, error::Error};

use advent_utils::{parse_file, Part, Solver};
use serde::Deserialize;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

mod movement;
mod simulator;

use movement::Movement;
use simulator::{Card, Deck, Math, Simulator};

const DECK_SIZE_PT1: usize = 10007;
const DECK_SIZE_PT2: usize = 119_315_717_514_047;
const CARD_TO_FIND: i64 = 2019;
const IDX_TO_LOOKUP: usize = 2020;

const ITERS_PT1: usize = 1;
// const ITERS_PT2: usize = 1;
const ITERS_PT2: usize = 101_741_582_076_661;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SimulationMode {
    Deck,
    Card,
    Math,
}

#[derive(Debug)]
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
            mode: SimulationMode::Math,
        })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        22
    }

    fn solve(&self, part: Part) -> String {
        let deck_size = match part {
            Part::One => DECK_SIZE_PT1,
            Part::Two => DECK_SIZE_PT2,
        };

        let iters = match part {
            Part::One => ITERS_PT1,
            Part::Two => ITERS_PT2,
        };

        let mut simulator: Box<dyn Simulator> = match self.mode {
            SimulationMode::Card => Box::new(Card::new(deck_size as i64, CARD_TO_FIND)),
            SimulationMode::Deck => Box::new(Deck::new(deck_size)),
            SimulationMode::Math => Box::new(Math::new(deck_size as i64)),
        };

        simulator.run(&self.movements, iters);

        match part {
            Part::One => format!(
                "card {} is at idx {}",
                CARD_TO_FIND,
                simulator.find_card(CARD_TO_FIND)
            ),
            Part::Two => format!(
                "there is card {} at idx {}",
                simulator.get_card_at_idx(IDX_TO_LOOKUP),
                IDX_TO_LOOKUP
            ),
        }
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
