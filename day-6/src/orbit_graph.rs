use std::{collections::HashMap, iter::FromIterator, ptr, str::FromStr};

const INITIAL_PLANET: &str = "COM";

#[derive(Debug)]
struct Planet {
    parent: Option<usize>,
    order: usize,
    system: Vec<usize>,
}

#[derive(Debug)]
pub struct Planets {
    planets: Vec<Planet>,
    map: HashMap<String, usize>,
}

impl Planets {
    pub fn orbit_count_checksums(&self) -> usize {
        self.planets.iter().map(|p| p.order).sum()
    }

    pub fn steps_to_lca(&self, planet_a: &str, planet_b: &str) -> Option<usize> {
        let idx_a = self.map.get(planet_a)?;
        let idx_b = self.map.get(planet_b)?;

        let mut steps = 0;

        let mut cur_a = &self.planets[*idx_a];
        let mut cur_b = &self.planets[*idx_b];

        while cur_a.order > cur_b.order {
            cur_a = &self.planets[cur_a.parent?];
            steps += 1;
        }

        while cur_b.order > cur_a.order {
            cur_b = &self.planets[cur_b.parent?];
            steps += 1;
        }

        while !ptr::eq(cur_a, cur_b) {
            cur_a = &self.planets[cur_a.parent?];
            cur_b = &self.planets[cur_b.parent?];

            steps += 2;
        }

        // -2 due to lack of need to travel
        // down from planet YOU and up to planet SAN
        Some(if steps >= 2 { steps - 2 } else { 0 })
    }
}

#[derive(Debug)]
pub struct Orbit {
    base: String,
    planet: String,
}

#[derive(Debug)]
pub enum OrbitParseError {
    WrongFormat,
}

impl FromStr for Orbit {
    type Err = OrbitParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split(')');

        let base = splitted
            .next()
            .ok_or(OrbitParseError::WrongFormat)?
            .to_owned();
        let planet = splitted
            .next()
            .ok_or(OrbitParseError::WrongFormat)?
            .to_owned();

        Ok(Self { base, planet })
    }
}

impl<'a> FromIterator<Orbit> for Planets {
    fn from_iter<I: IntoIterator<Item = Orbit>>(iter: I) -> Self {
        let mut planet_orbits = iter.into_iter().fold(HashMap::new(), |mut map, orbit| {
            let system = map.entry(orbit.base).or_insert_with(Vec::new);
            system.push(orbit.planet);

            map
        });

        let mut to_process = vec![(None, INITIAL_PLANET.to_owned(), 0)];
        let mut planets = Vec::new();
        let mut map = HashMap::new();

        while let Some((parent, name, order)) = to_process.pop() {
            planets.push(Planet {
                parent,
                order,
                system: Vec::new(),
            });

            let self_idx = planets.len() - 1;

            if let Some(parent) = parent {
                let parent: &mut Planet = &mut planets[parent];
                parent.system.push(self_idx);
            }

            if let Some(orbiting) = planet_orbits.remove(&name) {
                to_process.extend(
                    orbiting
                        .into_iter()
                        .map(|name| (Some(self_idx), name, order + 1)),
                );
            }

            map.insert(name, self_idx);
        }

        Self { planets, map }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_checksum() {
        let orbits = indoc!(
            "
            COM)B
            B)C
            C)D
            D)E
            E)F
            B)G
            G)H
            D)I
            E)J
            J)K
            K)L"
        )
        .split('\n')
        .map(|s| s.parse::<Orbit>().unwrap());

        let planets: Planets = orbits.collect();
        assert_eq!(planets.orbit_count_checksums(), 42);
    }

    #[test]
    fn test_steps_to_lca() {
        let orbits = indoc!(
            "
            COM)B
            B)C
            C)D
            D)E
            E)F
            B)G
            G)H
            D)I
            E)J
            J)K
            K)L
            K)YOU
            I)SAN"
        )
        .split('\n')
        .map(|s| s.parse::<Orbit>().unwrap());

        let planets: Planets = orbits.collect();
        assert_eq!(planets.steps_to_lca("YOU", "SAN"), Some(4));
    }
}
