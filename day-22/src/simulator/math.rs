use crate::{Movement, Simulator};

mod algo;
mod model;

use model::Model;

#[derive(Debug)]
pub struct Math {
    deck_size: i64,
    model: Model,
}

impl Math {
    pub fn new(deck_size: i64) -> Self {
        Self {
            deck_size,
            model: Model::new(deck_size),
        }
    }
}

impl Simulator for Math {
    fn execute(&mut self, movement: &Movement) {
        self.model.apply(movement);
    }

    fn run(&mut self, movements: &[Movement], n_times: usize) {
        for movement in movements {
            self.model.apply(movement);
        }
        self.model.recursive_apply(n_times);
    }

    fn find_card(&self, needle: i64) -> usize {
        self.model.get_position(needle) as usize
    }

    fn get_card_at_idx(&self, idx: usize) -> i64 {
        self.model.get_value(idx as i64)
    }
}
