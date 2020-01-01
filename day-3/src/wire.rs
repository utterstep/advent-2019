use std::{iter::FromIterator, str::FromStr};

use itertools::iproduct;

use crate::{
    direction::{Direction, ParseDirectionError},
    point::Point,
    segment::Segment,
};

#[derive(Debug)]
pub struct Wire {
    segments: Vec<Segment>,
    steps: Vec<i32>,
    sums: Vec<i32>,
}

impl Wire {
    pub fn intersections_with<'a>(&'a self, other: &'a Wire) -> impl Iterator<Item = Point> + 'a {
        iproduct!(self.segments.iter(), other.segments.iter())
            .filter_map(|(s1, s2)| s1.intersect(s2))
            .filter(|p| !p.is_central_port())
    }

    pub fn steps_to_intersections_with<'a>(
        &'a self,
        other: &'a Wire,
    ) -> impl Iterator<Item = (i32, Point)> + 'a {
        iproduct!(
            self.segments.iter().enumerate(),
            other.segments.iter().enumerate()
        )
        .filter_map(|((i, s1), (j, s2))| s1.intersect(s2).map(|p| (i, j, p)))
        .filter(|(_, _, p)| !p.is_central_port())
        .map(move |(i, j, p)| {
            // FIXME: use precomputed sums
            let steps_taken = self.sums.get(i - 1).unwrap_or(&0)
                + other.sums.get(j - 1).unwrap_or(&0)
                + self.segments[i].distance_to_point(p)
                + other.segments[j].distance_to_point(p);

            (steps_taken, p)
        })
    }
}

impl FromStr for Wire {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',').map(str::parse::<Direction>).collect()
    }
}

impl FromIterator<Direction> for Wire {
    fn from_iter<I: IntoIterator<Item = Direction>>(iter: I) -> Self {
        let segments_and_steps = iter.into_iter().fold(
            Vec::new(),
            |mut segments: Vec<(Segment, i32)>, direction| {
                let start = segments
                    .last()
                    .map(|s| s.0.end)
                    .unwrap_or_else(|| (0, 0).into());
                let distance = direction.distance();
                let end = start + direction;

                segments.push((Segment::new(start, end), distance));

                segments
            },
        );

        let (segments, steps): (_, Vec<_>) = segments_and_steps.into_iter().unzip();
        let sums = steps.iter().fold(Vec::new(), |mut sums: Vec<i32>, step| {
            sums.push(sums.last().unwrap_or(&0) + step);

            sums
        });

        Self { segments, steps, sums }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire_from_iterator() {
        let wire: Result<Wire, _> = "R8,U5,L5,D3".parse();

        assert!(wire.is_ok());
        assert_eq!(
            wire.unwrap().segments,
            vec![
                Segment::new((0, 0), (8, 0)),
                Segment::new((8, 0), (8, 5)),
                Segment::new((8, 5), (3, 5)),
                Segment::new((3, 5), (3, 2)),
            ]
        );
    }

    #[test]
    fn test_wire_intersections() {
        // example 1
        let w1: Wire = "R8,U5,L5,D3".parse().unwrap();
        let w2: Wire = "U7,R6,D4,L4".parse().unwrap();

        let mut intersections = w1.intersections_with(&w2).collect::<Vec<_>>();
        intersections.sort_by_key(Point::manhattan_to_zero);

        assert_eq!(intersections, vec![(3, 3).into(), (6, 5).into()],);

        // example 2
        let w1: Wire = "R75,D30,R83,U83,L12,D49,R71,U7,L72".parse().unwrap();
        let w2: Wire = "U62,R66,U55,R34,D71,R55,D58,R83".parse().unwrap();

        let distance = w1
            .intersections_with(&w2)
            .map(|p| p.manhattan_to_zero())
            .min();
        assert_eq!(distance, Some(159));

        // example 3
        let w1: Wire = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"
            .parse()
            .unwrap();
        let w2: Wire = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".parse().unwrap();

        let distance = w1
            .intersections_with(&w2)
            .map(|p| p.manhattan_to_zero())
            .min();
        assert_eq!(distance, Some(135));
    }

    #[test]
    fn test_wire_intersections_with_steps() {
        // example 1
        let w1: Wire = "R8,U5,L5,D3".parse().unwrap();
        let w2: Wire = "U7,R6,D4,L4".parse().unwrap();

        let steps = w1
            .steps_to_intersections_with(&w2)
            .min_by_key(|(steps, _)| *steps)
            .unwrap();
        assert_eq!(steps.0, 30);

        // example 2
        let w1: Wire = "R75,D30,R83,U83,L12,D49,R71,U7,L72".parse().unwrap();
        let w2: Wire = "U62,R66,U55,R34,D71,R55,D58,R83".parse().unwrap();

        let steps = w1
            .steps_to_intersections_with(&w2)
            .min_by_key(|(steps, _)| *steps)
            .unwrap();
        assert_eq!(steps.0, 610);

        // example 3
        let w1: Wire = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"
            .parse()
            .unwrap();
        let w2: Wire = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".parse().unwrap();

        let steps = w1
            .steps_to_intersections_with(&w2)
            .min_by_key(|(steps, _)| *steps)
            .unwrap();
        assert_eq!(steps.0, 410);
    }
}
