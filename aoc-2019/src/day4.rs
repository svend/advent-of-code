// https://adventofcode.com/2019/day/4

use std::collections::HashMap;

fn valid(pass: &[u8]) -> bool {
    let mut has_double = false;
    for digits in pass.windows(2) {
        if digits[1] < digits[0] {
            return false;
        }
        if digits[0] == digits[1] {
            has_double = true
        }
    }
    has_double
}

fn valid_2(pass: &[u8]) -> bool {
    let mut counts: HashMap<&u8, usize> = HashMap::new();
    let mut previous: Option<&u8> = None;
    for d in pass {
        if let Some(previous) = previous {
            if d < previous {
                return false;
            }
        }
        previous = Some(d);
        let count = counts.entry(d).or_insert(0);
        *count += 1;
    }
    counts.values().any(|count| count == &2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_input(s: &str) -> Vec<u8> {
        s.chars()
            .map(|c| c.to_string().parse::<u8>().unwrap())
            .collect()
    }

    fn my_input() -> std::ops::RangeInclusive<usize> {
        let input = include_str!("day4.input");
        let parts: Vec<_> = input
            .split("-")
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect();
        parts[0]..=parts[1]
    }

    #[test]
    fn test_valid() {
        let tests = &[("111111", true), ("223450", false), ("123789", false)];
        for (input, expected) in tests {
            let input = parse_input(input);
            assert_eq!(&valid(&input), expected);
        }

        let count = my_input()
            .map(|i| format!("{}", i))
            .map(|s| parse_input(&s))
            .map(|pass| valid(&pass))
            .filter(|b| *b)
            .collect::<Vec<_>>()
            .len();
        assert_eq!(count, 921);
    }

    #[test]
    fn test_valid_2() {
        let tests = &[("112233", true), ("123444", false), ("111122", true)];
        for (input, expected) in tests {
            let input = parse_input(input);
            assert_eq!(&valid_2(&input), expected);
        }

        let count = my_input()
            .map(|i| format!("{}", i))
            .map(|s| parse_input(&s))
            .map(|pass| valid_2(&pass))
            .filter(|b| *b)
            .collect::<Vec<_>>()
            .len();
        assert_eq!(count, 603);
    }
}
