use itertools::Itertools;
use std::collections::HashSet;

fn anti_unit(c: char) -> char {
    if c.is_ascii_lowercase() {
        c.to_ascii_uppercase()
    } else if c.is_ascii_uppercase() {
        c.to_ascii_lowercase()
    } else {
        panic!("invalid unit {}", c)
    }
}

fn react(cs: &[char]) -> Vec<char> {
    let mut reacted = Vec::new();
    let mut skip = false;

    for (c1, c2) in cs.iter().tuple_windows() {
        if skip {
            skip = false;
            continue;
        }
        if anti_unit(*c1) != *c2 {
            skip = false;
            reacted.push(*c1);
        } else {
            skip = true;
        }
    }

    if !skip {
        match cs.last() {
            Some(c) => reacted.push(*c),
            _ => {}
        }
    }

    if reacted != cs {
        react(&reacted)
    } else {
        reacted
    }
}

fn shortest_react(cs: &[char]) -> usize {
    let set: HashSet<_> = cs.iter().map(|c| c.to_ascii_lowercase()).collect();
    let rs: Vec<_> = set
        .iter()
        .map(|unit| {
            let cs: Vec<_> = cs
                .iter()
                .cloned()
                .filter(|c| c != unit && *c != anti_unit(*unit))
                .collect();
            react(&cs)
        })
        .collect();

    rs.iter().map(|r| r.len()).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_react() {
        let s = "";
        let cs: Vec<_> = s.chars().collect();
        let reacted = react(&cs);
        assert_eq!(reacted, "".chars().collect::<Vec<_>>());
        assert_eq!(reacted.len(), 0);

        let s = "aA";
        let cs: Vec<_> = s.chars().collect();
        let reacted = react(&cs);
        assert_eq!(reacted, "".chars().collect::<Vec<_>>());
        assert_eq!(reacted.len(), 0);

        let s = "dabAcCaCBAcCcaDA";
        let cs: Vec<_> = s.chars().collect();
        let reacted = react(&cs);
        assert_eq!(reacted, "dabCBAcaDA".chars().collect::<Vec<_>>());
        assert_eq!(reacted.len(), 10);

        let s = include_str!("5.input").trim_right();
        let cs: Vec<_> = s.chars().collect();
        let reacted = react(&cs);
        assert_eq!(reacted.len(), 10888);
    }

    #[test]
    fn test_longest_react() {
        let s = "dabAcCaCBAcCcaDA";
        let cs: Vec<_> = s.chars().collect();
        assert_eq!(shortest_react(&cs), 4);

        let s = include_str!("5.input").trim_right();
        let cs: Vec<_> = s.chars().collect();
        assert_eq!(shortest_react(&cs), 6952);
    }
}
