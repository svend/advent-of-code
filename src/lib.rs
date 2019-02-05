fn freq() -> i32 {
    let input = include_str!("1.input");
    input.lines().map(|l| l.parse::<i32>().unwrap()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freq() {
        assert_eq!(freq(), 543);
    }
}
