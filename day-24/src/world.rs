use std::{
    error::Error,
    fmt::{self, Display},
    str::FromStr,
};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct World(u32);

#[derive(Debug)]
pub enum WorldParseError {
    InvalidInputSize,
    InvalidInputValue(char),
}

impl Display for WorldParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WorldParseError::InvalidInputSize => "invalid input size".to_owned(),
                WorldParseError::InvalidInputValue(c) => format!("invalid value in input: {}", c),
            }
        )
    }
}

impl Error for WorldParseError {}

impl FromStr for World {
    type Err = WorldParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '.' => Ok(0u32),
                    '#' => Ok(1u32),
                    c => Err(WorldParseError::InvalidInputValue(c)),
                })
            })
            .enumerate()
            .try_fold(0, |world, (i, bug)| {
                let bug_in_the_world = bug?
                    .checked_shl(i as u32)
                    .ok_or(WorldParseError::InvalidInputSize)?;

                Ok(world ^ bug_in_the_world)
            })
            .map(World)
    }
}

macro_rules! mask {
    ($($idx: expr),*) => {
        0 $(| (1 << $idx))*
    };
}

impl World {
    pub fn biodiversity(self) -> u32 {
        self.0
    }

    pub fn step(self) -> Self {
        let current = self.0;

        let mut next = 0;

        for y in 0..5 {
            for x in 0..5 {
                let i = x + y * 5;

                let mask = match (x, y) {
                    (0, 0) => mask!(i + 1, i + 5),
                    (0, 4) => mask!(i - 5, i + 1),
                    (0, _) => mask!(i - 5, i + 1, i + 5),
                    (4, 0) => mask!(i - 1, i + 5),
                    (4, 4) => mask!(i - 1, i - 5),
                    (4, _) => mask!(i - 1, i - 5, i + 5),
                    (_, 0) => mask!(i - 1, i + 1, i + 5),
                    (_, 4) => mask!(i - 1, i - 5, i + 1),
                    (_, _) => mask!(i - 1, i - 5, i + 1, i + 5),
                };

                let alive = (current & mask).count_ones();

                let current_cell = current & (1 << i);

                if current_cell > 0 {
                    if alive == 1 {
                        next ^= current_cell;
                    }
                } else if alive == 1 || alive == 2 {
                    next ^= 1 << i;
                }
            }
        }

        Self(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_impl() {
        let world: World = indoc!(
            "
            .....
            .....
            .....
            #....
            .#..."
        )
        .parse()
        .unwrap();

        assert_eq!(world.biodiversity(), 2129920);
    }

    #[test]
    fn test_step() {
        let world_1: World = indoc!(
            "
            ....#
            #..#.
            #..##
            ..#..
            #...."
        )
        .parse()
        .unwrap();

        let world_2: World = indoc!(
            "
            #..#.
            ####.
            ###.#
            ##.##
            .##.."
        )
        .parse()
        .unwrap();

        assert_eq!(world_1.step(), world_2);
    }

    #[test]
    fn test_repeat_example() {
        use std::collections::HashSet;

        let mut existing = HashSet::new();

        let mut world: World = indoc!(
            "
            ....#
            #..#.
            #..##
            ..#..
            #...."
        )
        .parse()
        .unwrap();

        while existing.insert(world) {
            world = world.step();
        }

        assert_eq!(world.biodiversity(), 2129920);
    }
}
