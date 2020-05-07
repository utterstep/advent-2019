use crate::movement::Movement;

pub trait Simulator {
    fn execute(&mut self, movement: &Movement);

    fn get_position(&self) -> usize;
}
