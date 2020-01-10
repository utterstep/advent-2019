use std::{cmp::Ordering, str::FromStr};

const DIMENSIONS: usize = 3;

#[derive(Debug)]
struct Moon {
    coords: [i32; DIMENSIONS],
    velocities: [i32; DIMENSIONS],
}

impl Moon {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            coords: [x, y, z],
            velocities: [0; DIMENSIONS],
        }
    }

    pub fn energy(&self) -> i32 {
        self.coords.iter().map(|c| c.abs()).sum::<i32>()
            * self.velocities.iter().map(|v| v.abs()).sum::<i32>()
    }
}

#[derive(Debug)]
pub struct IncorrectMoonFormat {}

impl FromStr for Moon {
    type Err = IncorrectMoonFormat;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s
            .trim_matches('<')
            .trim_matches('>')
            .split(',')
            .filter_map(|coord| coord.split('=').nth(1)?.parse::<i32>().ok());

        let x = coords.next().ok_or_else(|| IncorrectMoonFormat {})?;
        let y = coords.next().ok_or_else(|| IncorrectMoonFormat {})?;
        let z = coords.next().ok_or_else(|| IncorrectMoonFormat {})?;

        Ok(Self::new(x, y, z))
    }
}

#[derive(Debug)]
pub struct System {
    moons: Vec<Moon>,
}

impl FromStr for System {
    type Err = IncorrectMoonFormat;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moons = s
            .split('\n')
            .map(|line| line.parse::<Moon>())
            .collect::<Result<_, _>>()?;

        Ok(Self { moons })
    }
}

impl System {
    pub fn advance(&mut self) {
        for i in 0..self.moons.len() {
            let mut gravities = [0; DIMENSIONS];
            let (left, right) = self.moons.split_at_mut(i);
            // safe to unwrap, split_at_mut leaves i-th element in the right part
            let (moon, right) = right.split_first_mut().unwrap();

            for other in left {
                moon.coords
                    .iter()
                    .zip(other.coords.iter())
                    .map(|(a, b)| a.cmp(b))
                    .zip(gravities.iter_mut())
                    .for_each(|(ord, gravity)| {
                        *gravity += {
                            match ord {
                                Ordering::Less => 1,
                                Ordering::Greater => -1,
                                Ordering::Equal => 0,
                            }
                        }
                    });
            }

            // duplication leads to better performance than chain :(
            for other in right {
                moon.coords
                    .iter()
                    .zip(other.coords.iter())
                    .map(|(a, b)| a.cmp(b))
                    .zip(gravities.iter_mut())
                    .for_each(|(ord, gravity)| {
                        *gravity += {
                            match ord {
                                Ordering::Less => 1,
                                Ordering::Greater => -1,
                                Ordering::Equal => 0,
                            }
                        }
                    });
            }

            moon.velocities
                .iter_mut()
                .zip(&gravities)
                .for_each(|(vel, gravity)| {
                    *vel += gravity;
                });
        }

        for moon in self.moons.iter_mut() {
            moon.coords
                .iter_mut()
                .zip(&moon.velocities)
                .for_each(|(coord, vel)| {
                    *coord += vel;
                });
        }
    }

    pub fn energy(&self) -> i32 {
        self.moons.iter().map(|m| m.energy()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn test_system_advance() {
        let mut system: System = indoc!(
            "
            <x=-1, y=0, z=2>
            <x=2, y=-10, z=-7>
            <x=4, y=-8, z=8>
            <x=3, y=5, z=-1>"
        )
        .parse()
        .unwrap();

        // After 1 step:
        // pos=<x= 2, y=-1, z= 1>, vel=<x= 3, y=-1, z=-1>
        // pos=<x= 3, y=-7, z=-4>, vel=<x= 1, y= 3, z= 3>
        // pos=<x= 1, y=-7, z= 5>, vel=<x=-3, y= 1, z=-3>
        // pos=<x= 2, y= 2, z= 0>, vel=<x=-1, y=-3, z= 1>
        system.advance();

        assert_eq!(system.moons[0].coords, [2, -1, 1]);
        assert_eq!(system.moons[0].velocities, [3, -1, -1]);

        assert_eq!(system.moons[1].coords, [3, -7, -4]);
        assert_eq!(system.moons[1].velocities, [1, 3, 3]);

        assert_eq!(system.moons[2].coords, [1, -7, 5]);
        assert_eq!(system.moons[2].velocities, [-3, 1, -3]);

        assert_eq!(system.moons[3].coords, [2, 2, 0]);
        assert_eq!(system.moons[3].velocities, [-1, -3, 1]);

        // After 10 steps:
        // pos=<x= 2, y= 1, z=-3>, vel=<x=-3, y=-2, z= 1>
        // pos=<x= 1, y=-8, z= 0>, vel=<x=-1, y= 1, z= 3>
        // pos=<x= 3, y=-6, z= 1>, vel=<x= 3, y= 2, z=-3>
        // pos=<x= 2, y= 0, z= 4>, vel=<x= 1, y=-1, z=-1>
        for _ in 0..9 {
            system.advance();
        }

        assert_eq!(system.moons[0].coords, [2, 1, -3]);
        assert_eq!(system.moons[0].velocities, [-3, -2, 1]);

        assert_eq!(system.moons[1].coords, [1, -8, 0]);
        assert_eq!(system.moons[1].velocities, [-1, 1, 3]);

        assert_eq!(system.moons[2].coords, [3, -6, 1]);
        assert_eq!(system.moons[2].velocities, [3, 2, -3]);

        assert_eq!(system.moons[3].coords, [2, 0, 4]);
        assert_eq!(system.moons[3].velocities, [1, -1, -1]);
    }

    #[test]
    fn test_energy() {
        let mut system: System = indoc!(
            "
            <x=-1, y=0, z=2>
            <x=2, y=-10, z=-7>
            <x=4, y=-8, z=8>
            <x=3, y=5, z=-1>"
        )
        .parse()
        .unwrap();

        for _ in 0..10 {
            system.advance();
        }

        assert_eq!(system.energy(), 179);

        let mut system: System = indoc!(
            "
            <x=-8, y=-10, z=0>
            <x=5, y=5, z=10>
            <x=2, y=-7, z=3>
            <x=9, y=-8, z=-3>"
        )
        .parse()
        .unwrap();

        for _ in 0..100 {
            system.advance();
        }

        assert_eq!(system.energy(), 1940);
    }
}
