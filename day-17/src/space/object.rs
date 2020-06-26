use std::{
    convert::TryFrom,
    fmt::{self, Display},
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

impl Orientation {
    pub fn left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }

    pub fn right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ObjectParseError {
    UnknownSymbol(u8),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Object {
    Empty,
    Scaffold,
    CleanScaffold,
    Robot(Orientation),
}

impl Object {
    pub fn get_orientation(&self) -> Option<Orientation> {
        match self {
            Self::Robot(orientation) => Some(*orientation),
            _ => None,
        }
    }
}

impl TryFrom<u8> for Object {
    type Error = ObjectParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'.' => Ok(Self::Empty),
            b'#' => Ok(Self::Scaffold),
            // ^, v, <, or >
            b'^' => Ok(Self::Robot(Orientation::Up)),
            b'>' => Ok(Self::Robot(Orientation::Right)),
            b'v' => Ok(Self::Robot(Orientation::Down)),
            b'<' => Ok(Self::Robot(Orientation::Left)),
            other => Err(ObjectParseError::UnknownSymbol(other)),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => '.',
                Self::Scaffold => '#',
                Self::CleanScaffold => '+',
                Self::Robot(Orientation::Up) => '^',
                Self::Robot(Orientation::Right) => '>',
                Self::Robot(Orientation::Down) => 'v',
                Self::Robot(Orientation::Left) => '<',
            }
        )
    }
}
