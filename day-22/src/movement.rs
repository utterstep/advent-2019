use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Movement {
    DealIn,
    DealWithIncrement(i64),
    Cut(i64),
}

#[derive(Debug, PartialEq, Eq)]
pub enum MovementParseError {
    UnknownMovement,
    IntParseError,
}

const DEAL_IN: &str = "deal into new stack";
const DEAL_WITH_INCREMENT: &str = "deal with increment";
const CUT: &str = "cut";

impl FromStr for Movement {
    type Err = MovementParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == DEAL_IN {
            Ok(Self::DealIn)
        } else if s.starts_with(DEAL_WITH_INCREMENT) {
            Ok(Self::DealWithIncrement(
                s[DEAL_WITH_INCREMENT.len() + 1..]
                    .parse()
                    .map_err(|_| MovementParseError::IntParseError)?,
            ))
        } else if s.starts_with(CUT) {
            Ok(Self::Cut(
                s[CUT.len() + 1..]
                    .parse()
                    .map_err(|_| MovementParseError::IntParseError)?,
            ))
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
