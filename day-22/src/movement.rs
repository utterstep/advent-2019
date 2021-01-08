use std::{num::ParseIntError, str::FromStr};

use displaydoc::Display;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Movement {
    DealIn,
    DealWithIncrement(i64),
    Cut(i64),
}

#[derive(Debug, PartialEq, Eq, Display, Error)]
pub enum MovementParseError {
    /// Unknown movements
    UnknownMovement,
    /// Error while parsing integer: {0}
    IntParseError(#[from] ParseIntError),
}

const DEAL_IN: &str = "deal into new stack";
const DEAL_WITH_INCREMENT: &str = "deal with increment ";
const CUT: &str = "cut ";

impl FromStr for Movement {
    type Err = MovementParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == DEAL_IN {
            Ok(Self::DealIn)
        } else if let Some(increment) = s.strip_prefix(DEAL_WITH_INCREMENT) {
            Ok(Self::DealWithIncrement(increment.parse()?))
        } else if let Some(cut) = s.strip_prefix(CUT) {
            Ok(Self::Cut(cut.parse()?))
        } else {
            Err(MovementParseError::UnknownMovement)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movement_parse() {
        assert_eq!("deal into new stack".parse(), Ok(Movement::DealIn));
        assert_eq!("cut -2".parse(), Ok(Movement::Cut(-2)));
        assert_eq!(
            "deal with increment 7".parse(),
            Ok(Movement::DealWithIncrement(7))
        );
        assert_eq!("cut 8".parse(), Ok(Movement::Cut(8)));
        assert_eq!("cut -4".parse(), Ok(Movement::Cut(-4)));
        assert_eq!("cut 3".parse(), Ok(Movement::Cut(3)));
        assert_eq!(
            "deal with increment 9".parse(),
            Ok(Movement::DealWithIncrement(9))
        );
        assert_eq!(
            "deal with increment 3".parse(),
            Ok(Movement::DealWithIncrement(3))
        );
        assert_eq!("cut -1".parse(), Ok(Movement::Cut(-1)));

        assert!("test".parse::<Movement>().is_err());
    }
}
