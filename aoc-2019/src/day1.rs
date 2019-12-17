fn fuel(mass: i32) -> i32 {
    let f = mass / 3 - 2;
    if f == 0 {
        return 0;
    }

    f
}

fn fuel_recursive_2(mass: i32) -> i32 {
    let mut f = fuel(mass);
    let mut ff = fuel(f);
    while { ff > 0 } {
        f += ff;
        ff = fuel(ff)
    }
    f
}

fn fuel_recursive(mass: i32) -> i32 {
    let f = fuel(mass);
    if f <= 0 {
        return 0;
    }
    f + fuel_recursive(f)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> impl Iterator<Item = i32> {
        let input = include_str!("day1.input");
        input.lines().map(|line| line.parse::<i32>().unwrap())
    }

    #[test]
    fn test_fuel() {
        let tests = &[(12, 2), (14, 2), (1969, 654), (100756, 33583)];
        for (input, expected) in tests {
            assert_eq!(&fuel(*input), expected);
        }

        let total_fuel: i32 = input().map(|mass| fuel(mass)).sum();
        assert_eq!(total_fuel, 3373568);
    }

    #[test]
    fn test_fuel_recursive() {
        let tests = &[(14, 2), (1969, 966), (100756, 50346)];
        for (input, expected) in tests {
            assert_eq!(&fuel_recursive(*input), expected);
            assert_eq!(&fuel_recursive_2(*input), expected);
        }

        let total_fuel: i32 = input().map(|mass| fuel_recursive(mass)).sum();
        assert_eq!(total_fuel, 5057481);
    }
}
