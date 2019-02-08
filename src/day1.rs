use std::collections::HashSet;

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
}
