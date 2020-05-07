use crate::{movement::Movement, traits::Simulator};

#[derive(Debug)]
pub struct Card {
    deck_size: i64,
    position: i64,
}

impl Card {
    pub fn new(deck_size: i64, position: i64) -> Self {
        Self {
            deck_size,
            position,
        }
    }
}

impl Simulator for Card {
    fn execute(&mut self, movement: &Movement) {
        self.position = match *movement {
            Movement::DealIn => self.deck_size - self.position - 1,
            Movement::DealWithIncrement(inc) => (self.position * inc) % self.deck_size,
            Movement::Cut(n) => (self.position - n).rem_euclid(self.deck_size),
        };
    }

    fn get_position(&self) -> usize {
        self.position as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card() {
        let mut card = Card::new(10, 7);
        card.execute(&Movement::DealIn);
        assert_eq!(card.get_position(), 2);

        let mut card = Card::new(10, 2);
        card.execute(&Movement::Cut(3));
        assert_eq!(card.get_position(), 9);

        let mut card = Card::new(10, 9);
        card.execute(&Movement::Cut(-4));
        assert_eq!(card.get_position(), 3);

        let mut card = Card::new(10, 8);
        card.execute(&Movement::DealWithIncrement(3));
        assert_eq!(card.get_position(), 4);
    }
}
