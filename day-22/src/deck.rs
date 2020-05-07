use std::collections::VecDeque;

use crate::{movement::Movement, traits::Simulator};

#[derive(Debug)]
pub struct Deck {
    cards: VecDeque<i64>,
    needle: i64,
}

impl Deck {
    pub fn new(size: usize, needle: i64) -> Self {
        assert!(needle < size as i64);

        let mut cards = VecDeque::with_capacity(size);
        (0..size as i64).for_each(|n| cards.push_back(n));

        Self { cards, needle }
    }
}

impl Simulator for Deck {
    fn execute(&mut self, movement: &Movement) {
        match movement {
            &Movement::DealIn => {
                let mut i = 0;
                let ln = self.cards.len();

                while i < ln / 2 {
                    self.cards.swap(i, ln - i - 1);

                    i += 1;
                }
            }
            &Movement::Cut(n) if n > 0 => {
                self.cards.rotate_left(n as usize);
            }
            &Movement::Cut(n) if n < 0 => {
                self.cards.rotate_right(-n as usize);
            }
            &Movement::Cut(n) if n == 0 => {}
            &Movement::Cut(_) => unreachable!(),
            &Movement::DealWithIncrement(increment) => {
                let len = self.cards.len();
                let mut cards = Vec::with_capacity(len);
                cards.resize(len, 0);

                let mut idx = 0;
                for card in &self.cards {
                    cards[idx] = *card;

                    idx += increment as usize;
                    if idx > len {
                        idx -= len;
                    }
                }

                self.cards = VecDeque::from(cards);
            }
        }
    }

    fn get_position(&self) -> usize {
        self.cards.iter().position(|&c| c == self.needle).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck() {
        let mut deck = Deck::new(10, 1);
        deck.execute(&Movement::DealIn);

        assert_eq!(
            deck.cards,
            VecDeque::from(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0])
        );

        let mut deck = Deck::new(10, 1);
        deck.execute(&Movement::Cut(3));

        assert_eq!(
            deck.cards,
            VecDeque::from(vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2])
        );

        let mut deck = Deck::new(10, 1);
        deck.execute(&Movement::Cut(-4));

        assert_eq!(
            deck.cards,
            VecDeque::from(vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5])
        );

        let mut deck = Deck::new(10, 1);
        deck.execute(&Movement::DealWithIncrement(3));

        assert_eq!(
            deck.cards,
            VecDeque::from(vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3])
        );
    }
}
