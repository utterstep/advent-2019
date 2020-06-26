use std::convert::TryFrom;

use crate::consts::*;

#[derive(Debug)]
pub enum Tile {
    Empty { x: usize, y: usize },
    Wall { x: usize, y: usize },
    Block { x: usize, y: usize },
    Paddle { x: usize, y: usize },
    Ball { x: usize, y: usize },
    SegmentDisplay { score: i64 },
}

impl Tile {
    pub fn get_coords(&self) -> Option<(usize, usize)> {
        match self {
            Self::Empty { x, y }
            | Self::Wall { x, y }
            | Self::Block { x, y }
            | Self::Paddle { x, y }
            | Self::Ball { x, y } => Some((*x, *y)),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Empty { x: _x, y: _y } => ' ',
            Self::Wall { x: _x, y: _y } => '⯀',
            Self::Block { x: _x, y: _y } => '🞙',
            Self::Paddle { x: _x, y: _y } => '▬',
            Self::Ball { x: _x, y: _y } => '🞄',
            _ => '?',
        }
    }
}

#[derive(Debug)]
pub enum TileParseError {
    InsufficientInput,
    InvalidTile { x: i64, y: i64, value: i64 },
}

impl<'a> TryFrom<&'a [i64]> for Tile {
    type Error = TileParseError;

    fn try_from(chunk: &[i64]) -> Result<Self, Self::Error> {
        let x = *chunk.get(0).ok_or(TileParseError::InsufficientInput)?;
        let y = *chunk.get(1).ok_or(TileParseError::InsufficientInput)?;
        let value = *chunk.get(2).ok_or(TileParseError::InsufficientInput)?;

        if (x, y) == (SEGMENT_X, SEGMENT_Y) {
            return Ok(Tile::SegmentDisplay { score: value });
        }

        let x_coord =
            usize::try_from(x).map_err(|_| TileParseError::InvalidTile { x, y, value })?;
        let y_coord =
            usize::try_from(y).map_err(|_| TileParseError::InvalidTile { x, y, value })?;

        let (x, y) = (x_coord, y_coord);

        debug_assert!(x <= MAX_X, "x is out of bounds: {}", x);
        debug_assert!(y <= MAX_Y, "y is out of bounds: {}", y);

        // 0 is an empty tile. No game object appears in this tile.
        // 1 is a wall tile. Walls are indestructible barriers.
        // 2 is a block tile. Blocks can be broken by the ball.
        // 3 is a horizontal paddle tile. The paddle is indestructible.
        // 4 is a ball tile. The ball moves diagonally and bounces off objects.
        match value {
            0 => Ok(Tile::Empty { x, y }),
            1 => Ok(Tile::Wall { x, y }),
            2 => Ok(Tile::Block { x, y }),
            3 => Ok(Tile::Paddle { x, y }),
            4 => Ok(Tile::Ball { x, y }),
            _ => Err(TileParseError::InvalidTile {
                x: x as i64,
                y: y as i64,
                value,
            }),
        }
    }
}
