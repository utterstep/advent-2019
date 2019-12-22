#[derive(Debug, PartialEq)]
pub enum Direction {
    Up(i32),
    Right(i32),
    Down(i32),
    Left(i32),
}

#[derive(Debug, PartialEq)]
pub enum ParseDirectionError {
    UnknownFormat,
}

impl std::str::FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = &s[..1];
        let distance = s[1..]
            .parse()
            .map_err(|_| ParseDirectionError::UnknownFormat)?;

        Ok(match direction {
            "U" => Direction::Up(distance),
            "R" => Direction::Right(distance),
            "D" => Direction::Down(distance),
            "L" => Direction::Left(distance),
            _ => return Err(ParseDirectionError::UnknownFormat),
        })
    }
}

impl Direction {
    pub fn distance(&self) -> i32 {
        match self {
            Self::Up(d) => *d,
            Self::Down(d) => *d,
            Self::Left(d) => *d,
            Self::Right(d) => *d,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_parse() {
        assert_eq!("U10".parse(), Ok(Direction::Up(10)));
        assert_eq!("D20".parse(), Ok(Direction::Down(20)));
        assert_eq!("R5".parse(), Ok(Direction::Right(5)));
        assert_eq!("L1337".parse(), Ok(Direction::Left(1337)));

        assert!("F1".parse::<Direction>().is_err());
        assert!("D".parse::<Direction>().is_err());
        assert!("UD1".parse::<Direction>().is_err());
        assert!("1D".parse::<Direction>().is_err());
        assert!("R1D".parse::<Direction>().is_err());
    }
}
