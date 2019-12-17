use std::collections::HashMap;
use std::convert;
use std::fmt;
use std::str::FromStr;

const RULE_LEN: usize = 5;
// const GENERATIONS: usize = 20;
const GENERATIONS: usize = 50_000_000_000;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Pot {
    NoPlant,
    Plant,
}

impl convert::From<char> for Pot {
    fn from(c: char) -> Self {
        match c {
            '.' => Pot::NoPlant,
            '#' => Pot::Plant,
            _ => panic!("invalid pot character"),
        }
    }
}

impl convert::From<Pot> for char {
    fn from(pot: Pot) -> Self {
        match pot {
            Pot::NoPlant => '.',
            Pot::Plant => '#',
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pots {
    pots: Vec<Pot>,
    first_idx: isize,
}

impl fmt::Display for Pots {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, pot) in self.pots.iter().cloned().enumerate() {
            let c: char = pot.into();
            if i as isize + self.first_idx == 0 {
                write!(f, "[{}]", c)?
            } else {
                write!(f, "{}", c)?
            }
        }
        Ok(())
    }
}

impl FromStr for Pots {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pots: Vec<_> = s.chars().map(|c| Pot::from(c)).collect();
        Ok(Pots {
            pots: pots,
            first_idx: 0,
        })
    }
}

impl Pots {
    fn sum_of_numbers(&self) -> isize {
        self.pots
            .iter()
            .cloned()
            .enumerate()
            .filter(|(_, pot)| pot == &Pot::Plant)
            .map(|(i, _)| i as isize + self.first_idx)
            .sum()
    }
}

fn parse_initial_state(s: &str) -> Pots {
    let fields: Vec<_> = s.split_whitespace().collect();
    let mut pots: Pots = fields[2].parse().unwrap();
    // add buffer around pots
    for _ in 0..(RULE_LEN - 1) * GENERATIONS {
        pots.first_idx -= 1;
        pots.pots.insert(0, Pot::NoPlant);
        pots.pots.push(Pot::NoPlant)
    }
    pots
}

type Rules = HashMap<Pots, Pot>;

fn parse_rules(lines: &[String]) -> Rules {
    lines
        .into_iter()
        .map(|l| {
            let fields: Vec<_> = l.splitn(2, "=>").collect();
            let pots: Pots = fields[0].parse().unwrap();
            let pot = Pot::from(fields[1].chars().next().unwrap());
            (pots, pot)
        })
        .collect()
}

fn parse_input(s: &str) -> (Pots, Rules) {
    let mut lines = s.lines();
    let pots = parse_initial_state(lines.next().unwrap());
    let lines: Vec<_> = lines
        .skip(1)
        .map(|l| l.split_whitespace().collect::<String>())
        .collect();
    let rules = parse_rules(&lines);
    (pots, rules)
}

fn sum_of_pots(pots: &mut Pots, rules: &Rules, generations: usize) -> isize {
    if generations == 0 {
        return pots.sum_of_numbers();
    }

    let first_idx = pots.first_idx;
    let pots: Vec<Pot> = pots
        .pots
        .windows(RULE_LEN)
        .map(|pots| {
            let pots = Pots {
                pots: pots.to_vec(),
                first_idx: 0,
            };
            rules.get(&pots).unwrap_or(&Pot::NoPlant)
        })
        .cloned()
        .collect();
    let mut pots = Pots {
        pots,
        first_idx: first_idx + 2,
    };
    sum_of_pots(&mut pots, rules, generations - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // let input = include_str!("12.example.input");
        // let (mut pots, rules) = parse_input(&input);
        // assert_eq!(sum_of_pots(&mut pots, &rules, GENERATIONS), 325);

        let input = include_str!("12.input");
        let (mut pots, rules) = parse_input(&input);
        assert_eq!(sum_of_pots(&mut pots, &rules, GENERATIONS), 3241);
    }
}
