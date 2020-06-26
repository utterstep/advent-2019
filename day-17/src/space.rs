use std::{
    convert::TryFrom,
    error::Error,
    fmt::{self, Display},
    ops::Index,
    str::FromStr,
};

use itertools::iproduct;

mod object;
mod traversal;

use object::{Object, ObjectParseError, Orientation};
pub use traversal::Command;

#[derive(Debug, PartialEq, Eq)]
pub enum SpaceParseError {
    UnevenLines,
    ObjectParseError(ObjectParseError),
}

impl Display for SpaceParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Error for SpaceParseError {}

impl From<ObjectParseError> for SpaceParseError {
    fn from(e: ObjectParseError) -> Self {
        Self::ObjectParseError(e)
    }
}

#[derive(Debug, Clone)]
pub struct Space {
    map: Vec<Object>,
    width: usize,
    height: usize,
}

impl Space {
    pub fn scaffold_alignment_parameters(&self) -> impl Iterator<Item = usize> + '_ {
        iproduct!(1..(self.height - 1), 1..(self.width - 1)).filter_map(move |(y, x)| {
            if self.is_scaffold_intersection(x, y) {
                Some(x * y)
            } else {
                None
            }
        })
    }

    fn is_scaffold_intersection(&self, x: usize, y: usize) -> bool {
        self[(x, y)] == Object::Scaffold
            && self[(x - 1, y)] == Object::Scaffold
            && self[(x, y - 1)] == Object::Scaffold
            && self[(x + 1, y)] == Object::Scaffold
            && self[(x, y + 1)] == Object::Scaffold
    }
}

impl Index<(usize, usize)> for Space {
    type Output = Object;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.map[x + y * self.width]
    }
}

impl FromStr for Space {
    type Err = SpaceParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = None;
        let mut height = 0;

        let lines_have_equal_length = s.lines().all(|line| {
            height += 1;

            if let Some(len) = width {
                line.len() == len
            } else {
                width.replace(line.len());

                true
            }
        });

        if !lines_have_equal_length {
            return Err(SpaceParseError::UnevenLines);
        }

        let map = s
            .lines()
            .flat_map(|line| line.bytes().map(TryFrom::try_from))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            map,
            height,
            width: width.unwrap_or(0),
        })
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let field = self
            .map
            .iter()
            .enumerate()
            .map(|(i, object)| {
                let (x, y) = (i % self.width, i / self.width);

                format!("{1}{0}", object, if x == 0 && y > 0 { "\n" } else { "" })
            })
            .collect::<String>();

        writeln!(f, "{}", field)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn test_space_parse() {
        let space: Space = indoc!(
            "
            ..#..........
            ..#..........
            #######...###
            #.#...#...#.#
            #############
            ..#...#...#..
            ..#####...^.."
        )
        .parse()
        .unwrap();

        assert_eq!(space.height, 7);
        assert_eq!(space.width, 13);

        let bad_space = ".#!".parse::<Space>();

        assert!(bad_space.is_err());
        assert_eq!(
            bad_space.unwrap_err(),
            SpaceParseError::ObjectParseError(ObjectParseError::UnknownSymbol(b'!'))
        );

        let uneven_space = indoc!(
            "
            .#.
            .<.."
        )
        .parse::<Space>();

        assert!(uneven_space.is_err());
        assert_eq!(uneven_space.unwrap_err(), SpaceParseError::UnevenLines);
    }

    #[test]
    fn test_scaffold_intersections() {
        let space: Space = indoc!(
            "
            ..#..........
            ..#..........
            #######...###
            #.#...#...#.#
            #############
            ..#...#...#..
            ..#####...^.."
        )
        .parse()
        .unwrap();

        assert_eq!(space.scaffold_alignment_parameters().sum::<usize>(), 76);
    }
}
