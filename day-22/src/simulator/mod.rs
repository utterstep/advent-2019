use enum_dispatch::enum_dispatch;

use crate::movement::Movement;

mod card;
mod deck;
mod math;

pub use card::Card;
pub use deck::Deck;
pub use math::Math;

#[enum_dispatch]
pub trait Simulator {
    fn execute(&mut self, movement: &Movement);

    fn run(&mut self, movements: &[Movement], n_times: usize) {
        for _ in 0..n_times {
            for movement in movements {
                self.execute(movement);
            }
        }
    }

    fn find_card(&self, needle: i64) -> usize;

    fn get_card_at_idx(&self, idx: usize) -> i64;
}
