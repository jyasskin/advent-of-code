use std::collections::HashMap;
use std::hash::Hash;
use std::io;
use std::io::Read;
use std::num::ParseIntError;
use std::str::FromStr;

const TRILLION: i64 = 1000i64 * 1000 * 1000 * 1000;

fn main() {
    let input = input();

    println!("Part 1: {}", compute_ore_needed(&input));

    println!("Part 2: {}", compute_fuel_for_ore(&input, TRILLION));
}

fn input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Couldn't read input");
    input
}

fn compute_ore_needed(input: &str) -> i64 {
    let reactions = parse(&input);
    compute_ore_needed_for_fuel(&reactions, 1)
}

fn compute_ore_needed_for_fuel(reactions: &Reactions, fuel: i64) -> i64 {
    let mut excess: HashMap<String, i64> = HashMap::new();
    compute_ore_needed_with_excess(&reactions, &mut excess, fuel)
}

fn compute_ore_needed_with_excess(
    reactions: &Reactions,
    excess: &mut HashMap<String, i64>,
    fuel: i64,
) -> i64 {
    let mut ore_needed: i64 = 0;
    let mut needed: HashMap<String, i64> = HashMap::new();
    needed.insert("FUEL".to_string(), fuel);
    // While there's an element in needed:
    while let Some((need_chemical, mut need_quantity)) = remove_one(&mut needed) {
        if need_chemical == "ORE" {
            ore_needed += need_quantity;
            continue;
        }
        let excess_chemical = excess.entry(need_chemical.clone()).or_insert(0);
        if *excess_chemical >= need_quantity {
            *excess_chemical -= need_quantity;
            continue;
        }
        need_quantity -= *excess_chemical;
        *excess_chemical = 0;
        let reaction = reactions.get(&need_chemical).unwrap();
        let (times, extra) = div_up(need_quantity, reaction.output.quantity);
        *excess_chemical = extra;
        for input in &reaction.inputs {
            *needed.entry(input.chemical.clone()).or_insert(0) += input.quantity * times;
        }
    }
    ore_needed
}

fn compute_fuel_for_ore(input: &str, ore: i64) -> i64 {
    let reactions = parse(&input);
    let mut low_fuel = 1;
    let mut high_fuel = 2;
    while compute_ore_needed_for_fuel(&reactions, high_fuel) < ore {
        high_fuel *= 10;
    }
    while high_fuel - low_fuel > 1 {
        let guess = (high_fuel + low_fuel) / 2;
        if compute_ore_needed_for_fuel(&reactions, guess) < ore {
            low_fuel = guess;
        } else {
            high_fuel = guess;
        }
    }
    low_fuel
}

// Returns the lowest number to multiple by piece_size to exceed quantity, and
// the amount of excess.
fn div_up(quantity: i64, piece_size: i64) -> (i64, i64) {
    let div = quantity / piece_size;
    let rem = quantity % piece_size;
    if rem == 0 {
        (div, rem)
    } else {
        (div + 1, piece_size - rem)
    }
}

fn remove_one<K: Clone + Eq + Hash, V>(map: &mut HashMap<K, V>) -> Option<(K, V)> {
    let key = {
        let key = map.iter().map(|(k, _)| k).cloned().next()?;
        Some(key)
    }?;
    let v = map.remove(&key).unwrap();
    Some((key, v))
}

#[derive(Clone, Debug)]
struct Reagent {
    quantity: i64,
    chemical: String,
}
impl FromStr for Reagent {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = s.trim().split_whitespace().collect();
        let quantity = fields[0].parse()?;
        let chemical = fields[1].to_string();
        Ok(Reagent { quantity, chemical })
    }
}

#[derive(Clone, Debug)]
struct Reaction {
    inputs: Vec<Reagent>,
    output: Reagent,
}
impl FromStr for Reaction {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let io: Vec<&str> = s.split("=>").collect();
        let inputs = io[0]
            .split(',')
            .map(|input| input.parse())
            .collect::<Result<Vec<Reagent>, ParseIntError>>()?;
        let output = io[1].parse()?;
        Ok(Reaction { inputs, output })
    }
}
type Reactions = HashMap<String, Reaction>;
fn parse(s: &str) -> Reactions {
    let mut result = HashMap::new();
    for line in s.lines() {
        let reaction: Reaction = line.parse().expect("Valid Reaction");
        result.insert(reaction.output.chemical.clone(), reaction.clone());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn examples() {
        assert_eq!(
            compute_ore_needed(
                "10 ORE => 10 A
        1 ORE => 1 B
        7 A, 1 B => 1 C
        7 A, 1 C => 1 D
        7 A, 1 D => 1 E
        7 A, 1 E => 1 FUEL"
            ),
            31
        );
        assert_eq!(
            compute_ore_needed(
                "9 ORE => 2 A
                8 ORE => 3 B
                7 ORE => 5 C
                3 A, 4 B => 1 AB
                5 B, 7 C => 1 BC
                4 C, 1 A => 1 CA
                2 AB, 3 BC, 4 CA => 1 FUEL"
            ),
            165
        );
        assert_eq!(
            compute_ore_needed(
                "157 ORE => 5 NZVS
                165 ORE => 6 DCFZ
                44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
                12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
                179 ORE => 7 PSHF
                177 ORE => 5 HKGWZ
                7 DCFZ, 7 PSHF => 2 XJWVT
                165 ORE => 2 GPVTF
                3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
            ),
            13312
        );
        assert_eq!(
            compute_ore_needed(
                "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
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
            180697
        );
        assert_eq!(
            compute_ore_needed(
                "171 ORE => 8 CNZTR
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
            2210736
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            compute_fuel_for_ore(
                "157 ORE => 5 NZVS
                165 ORE => 6 DCFZ
                44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
                12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
                179 ORE => 7 PSHF
                177 ORE => 5 HKGWZ
                7 DCFZ, 7 PSHF => 2 XJWVT
                165 ORE => 2 GPVTF
                3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
                TRILLION
            ),
            82892753
        );
        assert_eq!(
            compute_fuel_for_ore(
                "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
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
                176 ORE => 6 VJHF",
                TRILLION
            ),
            5586022
        );
        assert_eq!(
            compute_fuel_for_ore(
                "171 ORE => 8 CNZTR
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
                5 BHXH, 4 VRPVC => 5 LTCX",
                TRILLION
            ),
            460664
        );
    }
}
