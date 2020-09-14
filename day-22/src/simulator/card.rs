use crate::{Movement, Simulator};

#[derive(Debug)]
pub struct Card {
    deck_size: i64,
    current_position: i64,
    needle: i64,
}

impl Card {
    pub fn new(deck_size: i64, needle: i64) -> Self {
        Self {
            deck_size,
            current_position: needle,
            needle,
        }
    }
}

impl Simulator for Card {
    fn execute(&mut self, movement: &Movement) {
        self.current_position = match *movement {
            Movement::DealIn => self.deck_size - self.current_position - 1,
            Movement::DealWithIncrement(inc) => {
                (self.current_position * inc).rem_euclid(self.deck_size)
            }
            Movement::Cut(n) => (self.current_position - n).rem_euclid(self.deck_size),
        };
    }

    fn find_card(&self, needle: i64) -> usize {
        assert_eq!(needle, self.needle);

        self.current_position as usize
    }

    fn get_card_at_idx(&self, _idx: usize) -> i64 {
        unimplemented!("Card simulator can find only card it was created with :(")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card() {
        let mut card = Card::new(10, 7);
        card.execute(&Movement::DealIn);
        assert_eq!(card.find_card(7), 2);

        let mut card = Card::new(10, 2);
        card.execute(&Movement::Cut(3));
        assert_eq!(card.find_card(2), 9);

        let mut card = Card::new(10, 9);
        card.execute(&Movement::Cut(-4));
        assert_eq!(card.find_card(9), 3);

        let mut card = Card::new(10, 8);
        card.execute(&Movement::DealWithIncrement(3));
        assert_eq!(card.find_card(8), 4);
    }
}
