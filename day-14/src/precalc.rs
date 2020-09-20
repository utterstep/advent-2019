use std::{cmp::Ordering, convert::TryFrom};

use fnv::{FnvHashMap as Map, FnvHashSet as Set};

use super::reactions::{InvalidFormat, Reaction};

const FUEL: &str = "FUEL";
const ORE: &str = "ORE";

fn parse_input<'a>(recipe_str: &'a str) -> Result<Map<&'a str, Reaction<'a>>, InvalidFormat> {
    let mut map = Map::default();

    for line in recipe_str.split('\n') {
        let reaction = Reaction::try_from(line)?;

        let prev = map.insert(reaction.target.name, reaction);

        assert!(prev.is_none(), "ambigous recipe found")
    }

    Ok(map)
}

fn topological_sort<'a>(recipe: &Map<&'a str, Reaction<'a>>, start: &'a str) -> Vec<&'a str> {
    let mut order = vec![];
    let mut visited = Set::default();

    fn dfs<'a>(
        node: &'a str,
        order: &mut Vec<&'a str>,
        visited: &mut Set<&'a str>,
        recipe: &Map<&'a str, Reaction<'a>>,
    ) {
        if visited.insert(node) {
            for source in &recipe[node].sources {
                if source.name != ORE {
                    dfs(source.name, order, visited, recipe);
                }
            }

            order.push(node);
        }
    }

    dfs(start, &mut order, &mut visited, recipe);

    order.reverse();
    order.push(ORE);

    order
}

#[derive(Debug)]
pub struct SolutionPrecalc<'a> {
    recipe_data: Map<&'a str, Reaction<'a>>,
    material_order: Vec<&'a str>,
}

impl<'a> TryFrom<&'a str> for SolutionPrecalc<'a> {
    type Error = InvalidFormat;

    fn try_from(recipe_str: &'a str) -> Result<Self, Self::Error> {
        let recipe_data = parse_input(recipe_str)?;

        let material_order = topological_sort(&recipe_data, FUEL);

        Ok(SolutionPrecalc {
            recipe_data,
            material_order,
        })
    }
}

impl<'a> SolutionPrecalc<'a> {
    pub fn ore_requirements(&self, fuel_required: u64) -> u64 {
        let mut requirements = Map::default();
        requirements.insert(FUEL, fuel_required);

        for material in &self.material_order {
            if let Some(reaction) = self.recipe_data.get(material) {
                let count = requirements[material];
                let coeff = count / reaction.target.count
                    + if count % reaction.target.count > 0 {
                        1
                    } else {
                        0
                    };

                for source in &reaction.sources {
                    (*requirements.entry(source.name).or_insert(0)) += source.count * coeff;
                }
            }
        }

        requirements[ORE]
    }

    pub fn available_fuel(&self, available_ore: u64) -> u64 {
        let mut hi = available_ore;
        let mut lo = 1;

        loop {
            let mid = (hi + lo) / 2;
            let required_ore = self.ore_requirements(mid);

            match required_ore.cmp(&available_ore) {
                Ordering::Less => lo = mid,
                Ordering::Greater => hi = mid - 1,
                Ordering::Equal => return mid,
            };

            if hi - lo <= 1 {
                return lo;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const RECIPES: &[(&str, u64, u64)] = &[
        (
            indoc!(
                "
                9 ORE => 2 A
                8 ORE => 3 B
                7 ORE => 5 C
                3 A, 4 B => 1 AB
                5 B, 7 C => 1 BC
                4 C, 1 A => 1 CA
                2 AB, 3 BC, 4 CA => 1 FUEL"
            ),
            165,
            6323777402,
        ),
        (
            indoc!(
                "
                157 ORE => 5 NZVS
                165 ORE => 6 DCFZ
                44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
                12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
                179 ORE => 7 PSHF
                177 ORE => 5 HKGWZ
                7 DCFZ, 7 PSHF => 2 XJWVT
                165 ORE => 2 GPVTF
                3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
            ),
            13312,
            82892753,
        ),
        (
            indoc!(
                "
                2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
                17 NVRVD, 3 JNWZP => 8 VPVL
                53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
                22 VJHF, 37 MNCFX => 5 FWMGM
                139 ORE => 4 NVRVD
                144 ORE => 7 JNWZP
                5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
                5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
                145 ORE => 6 MNCFX
                1 NVRVD => 8 CXFTF
                1 VJHF, 6 MNCFX => 4 RFSQX
                176 ORE => 6 VJHF"
            ),
            180697,
            5586022,
        ),
        (
            indoc!(
                "
                171 ORE => 8 CNZTR
                7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
                114 ORE => 4 BHXH
                14 VRPVC => 6 BMBT
                6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
                6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
                15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
                13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
                5 BMBT => 4 WPTQ
                189 ORE => 9 KTJDG
                1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
                12 VRPVC, 27 CNZTR => 2 XDBXC
                15 KTJDG, 12 BHXH => 5 XCVML
                3 BHXH, 2 VRPVC => 7 MZWV
                121 ORE => 7 VRPVC
                7 XCVML => 6 RJRHP
                5 BHXH, 4 VRPVC => 5 LTCX"
            ),
            2210736,
            460664,
        ),
    ];

    #[test]
    fn test_examples_easy() {
        for (recipe, required, _) in RECIPES {
            let precalc = SolutionPrecalc::try_from(*recipe).unwrap();
            assert_eq!(precalc.ore_requirements(1), *required);
        }
    }

    #[test]
    fn test_examples_hard() {
        const ORE_QUANTITY: u64 = 1_000_000_000_000;

        for (recipe, _, available_fuel) in RECIPES {
            let precalc = SolutionPrecalc::try_from(*recipe).unwrap();
            assert_eq!(precalc.available_fuel(ORE_QUANTITY), *available_fuel);
        }
    }
}
