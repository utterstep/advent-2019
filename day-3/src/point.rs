use crate::direction::Direction;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct X(i32);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Y(i32);

macro_rules! bitxor_coord {
    ($type: ty) => {
        impl std::ops::BitXor for $type {
            type Output = $type;

            fn bitxor(self, rhs: Self) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }
    };
}

bitxor_coord!(X);
bitxor_coord!(Y);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: X,
    pub y: Y,
}

impl Point {
    pub fn is_central_port(&self) -> bool {
        self.x.0 == 0 && self.y.0 == 0
    }

    pub fn manhattan_distance_to(&self, other: Point) -> i32 {
        (self.x.0 - other.x.0).abs() + (self.y.0 - other.y.0).abs()
    }

    pub fn manhattan_to_zero(&self) -> i32 {
        self.x.0.abs() + self.y.0.abs()
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x: X(x), y: Y(y) }
    }
}

impl From<(X, Y)> for Point {
    fn from((x, y): (X, Y)) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up(distance) => (self.x.0, self.y.0 + distance).into(),
            Direction::Right(distance) => (self.x.0 + distance, self.y.0).into(),
            Direction::Down(distance) => (self.x.0, self.y.0 - distance).into(),
            Direction::Left(distance) => (self.x.0 - distance, self.y.0).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_add_direction() {
        macro_rules! test_add {
            ($start: expr, $dir: expr, $end: expr) => {
                let start: Point = $start.into();

                assert_eq!(start + $dir.parse::<Direction>().unwrap(), $end.into())
            };
        }

        test_add!((0, 0), "D10", (0, -10));
        test_add!((2, 1), "R15", (17, 1));
        test_add!((3, 6), "D10", (3, -4));
        test_add!((8, 22), "L10", (-2, 22));
        test_add!((8, 22), "U5", (8, 27));
    }

    #[test]
    fn test_manhattan_distance_to_zero() {
        macro_rules! test {
            ($point: expr, $expected: expr) => {
                let point: Point = $point.into();

                assert_eq!(point.manhattan_to_zero(), $expected);
            };
        }

        test!((0, 1), 1);
        test!((2, 4), 6);
        test!((-2, 22), 24);
        test!((0, 0), 0);
        test!((22, 11), 33);
    }
}
