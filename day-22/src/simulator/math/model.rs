use crate::Movement;

use advent_utils::algo::numeric as algo;

#[derive(Debug)]
/// Mathematical model of shuffle routine.
///
/// Target card position is `xM + N (mod deck_size)`
pub(super) struct Model {
    deck_size: i64,
    m: i64,
    n: i64,
}

impl Model {
    pub(super) fn new(deck_size: i64) -> Self {
        Self {
            n: 0,
            m: 1,
            deck_size,
        }
    }

    pub(super) fn apply(&mut self, movement: &Movement) {
        match movement {
            Movement::Cut(n) => self.cut(*n),
            Movement::DealIn => self.deal_in(),
            Movement::DealWithIncrement(m) => self.deal_inc(*m),
        }
    }

    fn cut(&mut self, n: i64) {
        self.n = (self.n - n) % self.deck_size;
    }

    fn deal_in(&mut self) {
        self.deal_inc(-1);
        self.cut(1);
    }

    fn deal_inc(&mut self, inc: i64) {
        self.m = (self.m * inc).rem_euclid(self.deck_size);
        self.n = (self.n * inc).rem_euclid(self.deck_size);
    }

    pub(super) fn get_position(&self, value: i64) -> i64 {
        (value as i128 * self.m as i128 + self.n as i128).rem_euclid(self.deck_size as i128) as i64
    }

    pub(super) fn get_value(&self, position: i64) -> i64 {
        let inverse_m = algo::inverse_mod_root(self.m, self.deck_size);

        algo::mult128(position - self.n, inverse_m, self.deck_size)
    }

    /// see `solution-thoughts.ipynb` for the details
    pub(super) fn recursive_apply(&mut self, n_times: usize) {
        let m_pow_n = algo::bin_pow(self.m, n_times, self.deck_size);
        let sum_m_pows_to_n_minus_one = algo::sum_of_pows(self.m, n_times - 1, self.deck_size);

        self.m = m_pow_n;
        self.n = algo::mult128(sum_m_pows_to_n_minus_one, self.n, self.deck_size);
    }

    #[cfg(test)]
    fn shuffle(&self) -> Vec<i64> {
        let mut with_order = (0..self.deck_size)
            .map(|n| (n, self.get_position(n)))
            .collect::<Vec<_>>();

        with_order.sort_unstable_by_key(|(_n, order)| *order);

        with_order.into_iter().map(|(n, _order)| n).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_examples() {
        // deal with increment 7
        // deal with increment 9
        // cut -2
        // Result: 6 3 0 7 4 1 8 5 2 9

        let mut m = Model::new(10);
        m.deal_inc(7);
        m.deal_inc(9);
        m.cut(-2);

        assert_eq!(&m.shuffle(), &[6, 3, 0, 7, 4, 1, 8, 5, 2, 9],);

        // cut 6
        // deal with increment 7
        // deal into new stack
        // Result: 3 0 7 4 1 8 5 2 9 6

        let mut m = Model::new(10);
        m.cut(6);
        m.deal_inc(7);
        m.deal_in();

        assert_eq!(&m.shuffle(), &[3, 0, 7, 4, 1, 8, 5, 2, 9, 6],);

        // deal into new stack
        // cut -2
        // deal with increment 7
        // cut 8
        // cut -4
        // deal with increment 7
        // cut 3
        // deal with increment 9
        // deal with increment 3
        // cut -1
        // Result: 9 2 5 8 1 4 7 0 3 6

        let mut m = Model::new(10);
        m.deal_in();
        m.cut(-2);
        m.deal_inc(7);
        m.cut(8);
        m.cut(-4);
        m.deal_inc(7);
        m.cut(3);
        m.deal_inc(9);
        m.deal_inc(3);
        m.cut(-1);

        assert_eq!(&m.shuffle(), &[9, 2, 5, 8, 1, 4, 7, 0, 3, 6],);
    }
}
