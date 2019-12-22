use std::error::Error;

use advent_utils::{get_config, parse_file, Part};

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

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;
    let modules = parse_file(config.input_file)?;

    match config.part {
        Part::One => {
            println!(
                "total fuel requirements: {}",
                modules.into_iter().map(get_fuel).sum::<u32>()
            );
        }
        Part::Two => println!(
            "total recursive fuel requirements: {}",
            modules
                .into_iter()
                .map(|module_weight| FuelRequirement::new(module_weight).sum::<u32>())
                .sum::<u32>()
        ),
    }

    Ok(())
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
