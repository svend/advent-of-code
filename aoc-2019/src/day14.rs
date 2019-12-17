// https://adventofcode.com/2019/day/14

use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

type Name = String;

#[derive(Debug, Clone)]
struct Chemical {
    name: Name,
    quantity: usize,
}

impl fmt::Display for Chemical {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.quantity, self.name)
    }
}

impl FromStr for Chemical {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();
        match parts.as_slice() {
            [quantity, name] => Ok(Self {
                name: name.to_string(),
                quantity: quantity.parse().unwrap(),
            }),
            _ => Err("invalid quantity".to_string()),
        }
    }
}

impl Chemical {
    fn new(name: &str, quantity: usize) -> Self {
        let name = name.to_string();
        Self { name, quantity }
    }
}

#[derive(Debug)]
struct Reaction {
    inputs: Vec<Chemical>,
    output: Chemical,
}

impl fmt::Display for Reaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, quantity) in self.inputs.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", quantity)?;
        }
        write!(f, " => {}", &self.output)
    }
}

impl FromStr for Reaction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.splitn(2, "=>").map(|s| s.trim()).collect();
        match parts.as_slice() {
            [inputs, output] => {
                let inputs: Vec<_> = inputs
                    .split(',')
                    .map(|s| s.trim())
                    .map(|s| s.parse().unwrap())
                    .collect();
                let output = output.parse().unwrap();
                Ok(Self { inputs, output })
            }
            _ => Err(format!("invalid reaction string: {}", s)),
        }
    }
}

fn parse_input(s: &str) -> Vec<Reaction> {
    s.trim().lines().map(|s| s.parse().unwrap()).collect()
}

fn get_secondary_chemicals(chemical: &Chemical, reactions: &[Reaction]) -> Vec<Chemical> {
    println!("OUTPUT: {}", chemical);
    let mut chemicals: Vec<Chemical> = Vec::new();
    for reaction in reactions
        .iter()
        .filter(|reaction| reaction.output.name == chemical.name)
    {
        // divide and round up
        let number =
            (chemical.quantity + (reaction.output.quantity - 1)) / reaction.output.quantity;
        println!("reaction ({}x): {}", number, reaction);
        for input in &reaction.inputs {
            if input.name == "ORE" {
                return vec![chemical.clone()];
            } else {
                println!("input: {}", input);
                let mut chems = get_secondary_chemicals(&input, reactions);
                for chemical in chems.iter_mut() {
                    chemical.quantity *= number;
                }
                chemicals.extend(chems);
            }
        }
    }
    chemicals
}

fn get_ore(chemical: &Chemical, reactions: &[Reaction]) -> usize {
    reactions
        .iter()
        .filter(|reaction| reaction.output.name == chemical.name)
        .filter(|reaction| reaction.inputs.iter().all(|input| input.name == "ORE"))
        .flat_map(|reaction| {
            let number =
                (chemical.quantity + (reaction.output.quantity - 1)) / reaction.output.quantity;
            reaction
                .inputs
                .iter()
                .map(move |input| input.quantity * number)
        })
        .sum()
}

fn reduce_chemicals(chemicals: &[Chemical]) -> Vec<Chemical> {
    let mut totals: HashMap<Name, usize> = HashMap::new();
    for chemical in chemicals {
        let entry = totals.entry(chemical.name.clone()).or_insert(0);
        *entry += chemical.quantity;
    }
    totals
        .into_iter()
        .map(|(name, quantity)| Chemical { name, quantity })
        .collect()
}

fn solve(reactions: &[Reaction]) -> usize {
    let fuel = Chemical::new("FUEL", 1);
    let chemicals = get_secondary_chemicals(&fuel, reactions);
    let chemicals = reduce_chemicals(&chemicals);
    chemicals
        .iter()
        .map(|chemical| get_ore(chemical, &reactions))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";

    const INPUT_2: &str = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";

    const INPUT_3: &str = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

    const INPUT_4: &str = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
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
176 ORE => 6 VJHF";

    const INPUT_5: &str = "171 ORE => 8 CNZTR
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
5 BHXH, 4 VRPVC => 5 LTCX";

    const MY_INPUT: &str = include_str!("day14.input");

    #[test]
    fn test_parse_reaction() {
        let inputs = &["10 ORE => 10 A"];
        for input in inputs {
            let reaction = Reaction::from_str(&input).unwrap();
            let formatted = format!("{}", reaction);
            assert_eq!(formatted, **input)
        }
    }

    #[test]
    fn test_example() {
        let tests = &[
            // (INPUT_1, 31),
            // (INPUT_2, 165),
            // (INPUT_3, 13312),
            (INPUT_4, 180697),
            (INPUT_5, 2210736),
            (MY_INPUT, 0),
        ];
        for (input, expected) in tests {
            let reactions = parse_input(input);
            assert_eq!(&solve(&reactions), expected);
        }
    }

    // #[test]
    // fn test_my_input() {
    //     let reactions = parse_input(MY_INPUT);
    //     for reaction in &reactions {
    //         println!("{}", reaction);
    //     }
    //     println!("***");
    //     find(&Chemical::new("FUEL", 1), &reactions);
    //     unimplemented!()
    // }
}
