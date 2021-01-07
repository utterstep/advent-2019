use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};

fn is_possible_password_weak(mut n: u32) -> bool {
    let mut current = n % 10;
    let mut have_double_digits = false;

    while n > 0 {
        n /= 10;

        let next = n % 10;
        if next > current {
            return false;
        }

        if next == current {
            have_double_digits = true;
        }

        current = next;
    }

    have_double_digits
}

fn is_possible_password_strong(mut n: u32) -> bool {
    let mut group_length = 1;
    let mut current = n % 10;
    let mut have_clear_double = false;

    while n > 0 {
        n /= 10;

        let next = n % 10;
        if next > current {
            return false;
        }

        if next == current {
            group_length += 1;
        } else {
            if group_length == 2 {
                have_clear_double = true;
            }

            group_length = 1;
        }

        current = next;
    }

    if group_length == 2 {
        have_clear_double = true;
    }

    have_clear_double
}

#[derive(Debug)]
pub struct Solution {
    min: u32,
    max: u32,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let mut parsed = input_data.trim().split('-').map(str::parse);

        let min = parsed.next().unwrap()?;
        let max = parsed.next().unwrap()?;

        Ok(Self { min, max })
    }
}

impl Solver for Solution {
    fn day_number() -> u32 {
        4
    }

    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => format!(
                "number of possible passwords: {}",
                (self.min..=self.max)
                    .filter(|n| is_possible_password_weak(*n))
                    .count()
            ),
            Part::Two => format!(
                "number of possible passwords: {}",
                (self.min..=self.max)
                    .filter(|n| is_possible_password_strong(*n))
                    .count()
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_possible_password_weak() {
        assert!(is_possible_password_weak(111111));
        assert!(is_possible_password_weak(122345));
        assert!(is_possible_password_weak(111123));
        assert!(!is_possible_password_weak(223450));
        assert!(!is_possible_password_weak(123789));
    }

    #[test]
    fn test_is_possible_password_strong() {
        assert!(is_possible_password_strong(122345));
        assert!(is_possible_password_strong(111122));
        assert!(!is_possible_password_strong(111123));
        assert!(!is_possible_password_strong(111111));
        assert!(!is_possible_password_strong(223450));
        assert!(!is_possible_password_strong(123789));
    }
}
