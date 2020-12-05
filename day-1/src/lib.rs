use std::{convert::TryFrom, error::Error, path::PathBuf};

use advent_utils::{parse_file, Part, Solver};

#[inline]
fn get_fuel(weight: u32) -> u32 {
    (weight / 3).saturating_sub(2)
}

struct FuelRequirement {
    unprovisioned_weight: u32,
}

impl FuelRequirement {
    fn new(weight: u32) -> Self {
        Self {
            unprovisioned_weight: weight,
        }
    }
}

impl Iterator for FuelRequirement {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let required_fuel = get_fuel(self.unprovisioned_weight);

        match required_fuel {
            0 => None,
            n => {
                self.unprovisioned_weight = n;

                Some(n)
            }
        }
    }
}

#[derive(Debug)]
pub struct Solution {
    modules: Vec<u32>,
}

impl TryFrom<PathBuf> for Solution {
    type Error = Box<dyn Error>;

    fn try_from(input_file: PathBuf) -> Result<Self, Self::Error> {
        let modules = parse_file(input_file)?;

        Ok(Self { modules })
    }
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => format!(
                "total fuel requirements: {}",
                self.modules.iter().cloned().map(get_fuel).sum::<u32>()
            ),

            Part::Two => format!(
                "total recursive fuel requirements: {}",
                self.modules
                    .iter()
                    .cloned()
                    .map(|module_weight| FuelRequirement::new(module_weight).sum::<u32>())
                    .sum::<u32>()
            ),
        }
    }

    fn day_number() -> u32 {
        1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_fuel() {
        assert_eq!(get_fuel(12), 2);
        assert_eq!(get_fuel(14), 2);
        assert_eq!(get_fuel(1969), 654);
        assert_eq!(get_fuel(100756), 33583);

        // assumtion
        assert_eq!(get_fuel(1), 0);
    }
}
