use std::collections::{HashMap, HashSet};

fn freq(nums: &[String]) -> i32 {
    nums.iter().map(|n| n.parse::<i32>().unwrap()).sum()
}

fn freq_2(nums: &[String]) -> i32 {
    let freqs: Vec<_> = nums.iter().map(|n| n.parse::<i32>().unwrap()).collect();

    let mut sum = 0;
    let mut seen = HashSet::new();

    loop {
        for freq in &freqs {
            if seen.contains(&sum) {
                return sum;
            }
            seen.insert(sum);
            sum += freq;
        }
    }
}

fn appears_n(s: &str, n: i32) -> bool {
    let mut counts = HashMap::new();

    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    counts.values().find(|&&i| i == n).is_some()
}

fn checksum(ids: &[String]) -> usize {
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

fn combinations(ids: &[String]) -> Vec<(String, String)> {
    let mut cs: Vec<(String, String)> = vec![];
    for (i, id1) in ids.iter().enumerate() {
        for id2 in ids.iter().skip(i + 1) {
            cs.push((id1.to_owned(), id2.to_owned()))
        }
    }
    cs
}

fn close_ids(ids: &[String]) -> Vec<(String, String)> {
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
    fn test_freq() {
        let nums: Vec<_> = ["+1", "+1", "+1"].iter().map(|s| s.to_string()).collect();
        assert_eq!(freq(&nums), 3);

        let input = include_str!("1.input");
        let nums: Vec<_> = input.lines().map(|s| s.to_string()).collect();
        assert_eq!(freq(&nums), 543);
    }

    #[test]
    fn test_freq_2() {
        let nums: Vec<_> = ["+1", "-1"].iter().map(|s| s.to_string()).collect();
        assert_eq!(freq_2(&nums), 0);

        let nums: Vec<_> = ["+3", "+3", "+4", "-2", "-4"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(freq_2(&nums), 10);

        let input = include_str!("1.input");
        let nums: Vec<_> = input.lines().map(|s| s.to_string()).collect();
        assert_eq!(freq_2(&nums), 621);
    }

    #[test]
    fn test_appears_n() {
        assert_eq!(appears_n("aabcdefg", 2), true);
        assert_eq!(appears_n("aabcdefg", 3), false);
    }

    #[test]
    fn test_checksum() {
        let ids: Vec<_> = [
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(checksum(&ids), 12);

        let input = include_str!("2.input");
        let ids: Vec<_> = input.lines().map(|s| s.to_string()).collect();
        assert_eq!(checksum(&ids), 8820);
    }

    #[test]
    fn test_differs_by() {
        assert_eq!(differs_by("abcde", "axcye"), 2);
        assert_eq!(differs_by("fghij", "fguij"), 1);
    }

    #[test]
    fn test_close_ids() {
        let ids: Vec<_> = [
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let close = close_ids(&ids);
        let (id1, id2) = close.first().unwrap();
        assert_eq!(common_chars(&id1, &id2), "fgij");

        let input = include_str!("2.input");
        let ids: Vec<_> = input.lines().map(|s| s.to_string()).collect();
        let close = close_ids(&ids);
        let (id1, id2) = close.first().unwrap();
        assert_eq!(common_chars(&id1, &id2), "bpacnmglhizqygfsjixtkwudr");
    }

    #[test]
    fn test_foo() {
        let input = include_str!("3.input");
        let claims: Vec<_> = input.lines().map(|s| s.to_string()).collect();
        for c in claims {
            println!("{:?}", c.split_whitespace())
        }
        unimplemented!()
    }
}
