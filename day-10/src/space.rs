use fnv::FnvHashSet;

use crate::rational::RationalAngle;

const ASTEROID: char = '#';

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Asteroid {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub(crate) struct Space {
    asteroids: Vec<Asteroid>,
}

impl Space {
    fn asteroids_seen_from(&self, location: &Asteroid) -> usize {
        self.asteroids
            .iter()
            .filter_map(|asteroid| {
                RationalAngle::new(asteroid.x - location.x, asteroid.y - location.y)
            })
            .collect::<FnvHashSet<_>>()
            .len()
    }

    pub fn best_station_location(&self) -> Option<(&Asteroid, usize)> {
        self.asteroids
            .iter()
            .map(|location| (location, self.asteroids_seen_from(location)))
            .max_by_key(|(_location, asteroids_seen)| *asteroids_seen)
    }

    pub fn asteroids_in_vaporize_order(
        &self,
        station: &Asteroid,
    ) -> impl Iterator<Item = &Asteroid> {
        let mut to_destroy = self
            .asteroids
            .iter()
            .filter_map(|asteroid| {
                let x = asteroid.x - station.x;
                let y = asteroid.y - station.y;
                let distance_sq = x * x + y * y;

                RationalAngle::new(x, y).map(|r| (asteroid, r, distance_sq))
            })
            .collect::<Vec<_>>();
        // safe to unwrap here ­— RationalAngle struct performs checks against corner cases
        to_destroy.sort_by(|a, b| (a.1.angle(), a.2).partial_cmp(&(b.1.angle(), b.2)).unwrap());

        let mut prev = None;
        let mut to_destroy = to_destroy
            .into_iter()
            .map(|(asteroid, r, _)| {
                let mut order = 1;

                if let Some((prev_r, prev_order)) = prev {
                    if prev_r == r {
                        order = prev_order + 1;
                    }
                }

                prev = Some((r, order));

                (asteroid, order)
            })
            .collect::<Vec<_>>();
        to_destroy.sort_by_key(|(_asteroid, order)| *order);

        to_destroy.into_iter().map(|(asteroid, _order)| asteroid)
    }
}

impl From<&str> for Space {
    fn from(s: &str) -> Self {
        let asteroids = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, ch)| {
                    if ch == ASTEROID {
                        let x = x as i32;
                        let y = y as i32;

                        Some(Asteroid { x, y })
                    } else {
                        None
                    }
                })
            })
            .collect();

        Self { asteroids }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_from_str() {
        let space = indoc!(
            "
            .#..#
            .....
            #####
            ....#
            ...##"
        );

        let asteroids = vec![
            Asteroid { x: 1, y: 0 },
            Asteroid { x: 4, y: 0 },
            Asteroid { x: 0, y: 2 },
            Asteroid { x: 1, y: 2 },
            Asteroid { x: 2, y: 2 },
            Asteroid { x: 3, y: 2 },
            Asteroid { x: 4, y: 2 },
            Asteroid { x: 4, y: 3 },
            Asteroid { x: 3, y: 4 },
            Asteroid { x: 4, y: 4 },
        ];

        assert_eq!(Space::from(space).asteroids, asteroids);
    }

    #[test]
    fn test_station_location() {
        macro_rules! assert_best_location {
            ($space: expr, $expected: expr) => {
                let space = Space::from(indoc!($space));

                assert_eq!(
                    space
                        .best_station_location()
                        .map(|(loc, seen_from)| ((loc.x, loc.y), seen_from)),
                    Some($expected)
                );
            };
        }

        assert_best_location!(
            "
            .#..#
            .....
            #####
            ....#
            ...##",
            ((3, 4), 8)
        );

        assert_best_location!(
            "
            ......#.#.
            #..#.#....
            ..#######.
            .#.#.###..
            .#..#.....
            ..#....#.#
            #..#....#.
            .##.#..###
            ##...#..#.
            .#....####",
            ((5, 8), 33)
        );

        assert_best_location!(
            "
            #.#...#.#.
            .###....#.
            .#....#...
            ##.#.#.#.#
            ....#.#.#.
            .##..###.#
            ..#...##..
            ..##....##
            ......#...
            .####.###.",
            ((1, 2), 35)
        );

        assert_best_location!(
            "
            .#..#..###
            ####.###.#
            ....###.#.
            ..###.##.#
            ##.##.#.#.
            ....###..#
            ..#.#..#.#
            #..#.#.###
            .##...##.#
            .....#.#..",
            ((6, 3), 41)
        );

        assert_best_location!(
            "
            .#..##.###...#######
            ##.############..##.
            .#.######.########.#
            .###.#######.####.#.
            #####.##.#.##.###.##
            ..#####..#.#########
            ####################
            #.####....###.#.#.##
            ##.#################
            #####.##.###..####..
            ..######..##.#######
            ####.##.####...##..#
            .#####..#.######.###
            ##...#.##########...
            #.##########.#######
            .####.#.###.###.#.##
            ....##.##.###..#####
            .#.#.###########.###
            #.#.#.#####.####.###
            ###.##.####.##.#..##",
            ((11, 13), 210)
        );
    }

    #[test]
    fn test_vaporization_order() {
        let space: Space = indoc!(
            "
            .#..##.###...#######
            ##.############..##.
            .#.######.########.#
            .###.#######.####.#.
            #####.##.#.##.###.##
            ..#####..#.#########
            ####################
            #.####....###.#.#.##
            ##.#################
            #####.##.###..####..
            ..######..##.#######
            ####.##.####...##..#
            .#####..#.######.###
            ##...#.##########...
            #.##########.#######
            .####.#.###.###.#.##
            ....##.##.###..#####
            .#.#.###########.###
            #.#.#.#####.####.###
            ###.##.####.##.#..##"
        )
        .into();

        let station = space.best_station_location().unwrap().0;
        dbg!(&station);
        let asteroids_in_vaporize_order: Vec<_> =
            space.asteroids_in_vaporize_order(station).collect();

        macro_rules! assert_vaporized {
            ($number: expr, $coords: expr) => {
                assert_eq!(
                    asteroids_in_vaporize_order
                        .get($number - 1)
                        .map(|a| (a.x, a.y)),
                    Some($coords)
                );
            };
        }

        // The 1st asteroid to be vaporized is at 11,12.
        // The 2nd asteroid to be vaporized is at 12,1.
        // The 3rd asteroid to be vaporized is at 12,2.
        // The 10th asteroid to be vaporized is at 12,8.
        // The 20th asteroid to be vaporized is at 16,0.
        // The 50th asteroid to be vaporized is at 16,9.
        // The 100th asteroid to be vaporized is at 10,16.
        // The 199th asteroid to be vaporized is at 9,6.
        // The 200th asteroid to be vaporized is at 8,2.
        // The 201st asteroid to be vaporized is at 10,9.
        // The 299th and final asteroid to be vaporized is at 11,1.

        assert_vaporized!(1, (11, 12));
        assert_vaporized!(2, (12, 1));
        assert_vaporized!(3, (12, 2));
        assert_vaporized!(10, (12, 8));
        assert_vaporized!(20, (16, 0));
        assert_vaporized!(50, (16, 9));
        assert_vaporized!(100, (10, 16));
        assert_vaporized!(199, (9, 6));
        assert_vaporized!(200, (8, 2));
        assert_vaporized!(201, (10, 9));
        assert_vaporized!(299, (11, 1));
    }
}
