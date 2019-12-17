use std::collections::HashMap;

fn appears_n(s: &str, n: i32) -> bool {
    let mut counts = HashMap::new();

    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    counts.values().find(|&&i| i == n).is_some()
}

fn checksum(ids: &[&str]) -> usize {
    let two_times = ids.iter().filter(|s| appears_n(s, 2)).count();
    let three_times = ids.iter().filter(|s| appears_n(s, 3)).count();
    two_times * three_times
}

fn differs_by(s1: &str, s2: &str) -> usize {
    s1.chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

fn combinations(ids: &[&str]) -> Vec<(String, String)> {
    let mut cs = vec![];
    for (i, id1) in ids.iter().enumerate() {
        for id2 in ids.iter().skip(i + 1) {
            cs.push((id1.to_string(), id2.to_string()))
        }
    }
    cs
}

fn close_ids(ids: &[&str]) -> Vec<(String, String)> {
    combinations(ids)
        .into_iter()
        .filter(|(id1, id2)| differs_by(id1, id2) == 1)
        .collect()
}

fn common_chars(s1: &str, s2: &str) -> String {
    s1.chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|(c1, _)| c1)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_appears_n() {
        assert_eq!(appears_n("aabcdefg", 2), true);
        assert_eq!(appears_n("aabcdefg", 3), false);
    }

    #[test]
    fn test_checksum() {
        let ids = [
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];
        assert_eq!(checksum(&ids), 12);

        let input = include_str!("2.input");
        let ids: Vec<_> = input.lines().collect();
        assert_eq!(checksum(&ids), 8820);
    }

    #[test]
    fn test_differs_by() {
        assert_eq!(differs_by("abcde", "axcye"), 2);
        assert_eq!(differs_by("fghij", "fguij"), 1);
    }

    #[test]
    fn test_close_ids() {
        let ids = [
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ];
        let close = close_ids(&ids);
        let (id1, id2) = close.first().unwrap();
        assert_eq!(common_chars(&id1, &id2), "fgij");

        let input = include_str!("2.input");
        let ids: Vec<_> = input.lines().collect();
        let close = close_ids(&ids);
        let (id1, id2) = close.first().unwrap();
        assert_eq!(common_chars(&id1, &id2), "bpacnmglhizqygfsjixtkwudr");
    }
}
