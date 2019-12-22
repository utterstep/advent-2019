use crate::point::Point;

#[derive(Debug, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

/// Segment with 90-degree orientation
#[derive(Debug, PartialEq)]
pub struct Segment {
    pub start: Point,
    pub end: Point,
    orientation: Orientation,
}

impl Segment {
    pub fn new<T: Into<Point>>(start: T, end: T) -> Self {
        let start = start.into();
        let end = end.into();
        let orientation = if start.x == end.x {
            Orientation::Vertical
        } else {
            Orientation::Horizontal
        };

        Self {
            start,
            end,
            orientation,
        }
    }

    pub fn intersect(&self, other: &Self) -> Option<Point> {
        if self.orientation == other.orientation {
            return None;
        }

        let is_self_vertical = self.orientation == Orientation::Vertical;

        let vertical = if is_self_vertical { self } else { other };
        let horizontal = if is_self_vertical { other } else { self };

        let x_v = vertical.start.x;
        let y_h = horizontal.start.y;

        let x_1 = horizontal.start.x.min(horizontal.end.x);
        let x_2 = x_1 ^ horizontal.start.x ^ horizontal.end.x;

        let y_1 = vertical.start.y.min(vertical.end.y);
        let y_2 = y_1 ^ vertical.start.y ^ vertical.end.y;

        if x_1 <= x_v && x_v <= x_2 && y_1 <= y_h && y_h <= y_2 {
            Some((x_v, y_h).into())
        } else {
            None
        }
    }

    pub fn distance_to_point(&self, point: Point) -> i32 {
        self.start.manhattan_distance_to(point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_new() {
        let segment = Segment::new((0, 0), (1, 0));
        assert_eq!(segment.orientation, Orientation::Horizontal);

        let segment = Segment::new((2, 0), (2, 20));
        assert_eq!(segment.orientation, Orientation::Vertical);
    }

    #[test]
    fn test_segment_intersection() {
        let s1 = Segment::new((0, 0), (0, 5));
        let s2 = Segment::new((-1, 4), (5, 4));

        assert_eq!(s1.intersect(&s2), Some((0, 4).into()));

        let s1 = Segment::new((0, 0), (0, 5));
        let s2 = Segment::new((-1, 20), (5, 20));

        assert_eq!(s1.intersect(&s2), None);

        let s1 = Segment::new((0, 0), (0, 5));
        let s2 = Segment::new((-1, 5), (20, 5));

        assert_eq!(s1.intersect(&s2), Some((0, 5).into()));
    }
}
