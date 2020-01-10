use std::{convert::TryFrom, iter::FromIterator, ptr};

use fnv::FnvHashMap;

const INITIAL_PLANET: &str = "COM";

#[derive(Debug)]
struct Planet {
    parent: Option<usize>,
    order: usize,
    system: Vec<usize>,
}

#[derive(Debug)]
pub struct Planets<'a> {
    planets: Vec<Planet>,
    // use slightly faster (and less secure, but here it doesn't matter) FNV hash
    names_map: FnvHashMap<&'a str, usize>,
}

impl<'a> Planets<'a> {
    pub fn orbit_count_checksums(&self) -> usize {
        self.planets.iter().map(|p| p.order).sum()
    }

    pub fn steps_to_lca(&self, planet_a: &str, planet_b: &str) -> Option<usize> {
        let idx_a = self.names_map.get(planet_a)?;
        let idx_b = self.names_map.get(planet_b)?;

        let mut steps = 0;

        let mut cur_a = &self.planets[*idx_a];
        let mut cur_b = &self.planets[*idx_b];

        // advance up, as we need to get from orbit to orbit, not from planet to planet
        if let Some(parent) = cur_a.parent {
            cur_a = &self.planets[parent];
        }
        if let Some(parent) = cur_b.parent {
            cur_b = &self.planets[parent];
        }

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

        Some(steps)
    }
}

#[derive(Debug)]
pub struct Orbit<'a> {
    base: &'a str,
    planet: &'a str,
}

#[derive(Debug)]
pub enum OrbitParseError {
    WrongFormat,
}

impl<'a> TryFrom<&'a str> for Orbit<'a> {
    type Error = OrbitParseError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut splitted = s.split(')');

        let base = splitted.next().ok_or(OrbitParseError::WrongFormat)?;
        let planet = splitted.next().ok_or(OrbitParseError::WrongFormat)?;

        Ok(Self { base, planet })
    }
}

impl<'a> FromIterator<Orbit<'a>> for Planets<'a> {
    fn from_iter<I: IntoIterator<Item = Orbit<'a>>>(iter: I) -> Self {
        let mut planet_orbits = iter.into_iter().fold(
            FnvHashMap::with_capacity_and_hasher(1600, Default::default()),
            |mut map, orbit| {
                let system = map.entry(orbit.base).or_insert_with(Vec::new);
                system.push(orbit.planet);

                map
            },
        );

        let mut to_process = vec![(None, INITIAL_PLANET, 0)];
        let mut planets = Vec::new();
        let mut names_map = FnvHashMap::with_capacity_and_hasher(1600, Default::default());

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

            names_map.insert(name, self_idx);
        }

        Self { planets, names_map }
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
        .map(|s| Orbit::try_from(s).unwrap());

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
        .map(|s| Orbit::try_from(s).unwrap());

        let planets: Planets = orbits.collect();
        assert_eq!(planets.steps_to_lca("YOU", "SAN"), Some(4));
    }
}
