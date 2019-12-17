// https://adventofcode.com/2019/day/16

const BASE_PATTERN: &[i32] = &[0, 1, 0, -1];

fn pattern(repeat: usize) -> impl Iterator<Item = i32> {
    assert!(repeat != 0);
    BASE_PATTERN
        .iter()
        .cloned()
        .flat_map(move |n| std::iter::repeat(n).take(repeat))
        .cycle()
}

/// Returns the ones digit for a number
fn ones(n: i32) -> i32 {
    (n % 10).abs()
}

fn parse_input(s: &str) -> Vec<u32> {
    s.trim().chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn transform(input: &[u32]) -> Vec<u32> {
    let mut output = Vec::with_capacity(input.len());
    for i in 0..input.len() {
        let sum = input
            .iter()
            .zip(pattern(i + 1).skip(1))
            .map(|(i, j)| (*i as i32) * j)
            .sum();
        output.push(ones(sum) as u32)
    }
    output
}

fn transform_iter(input: Vec<u32>) -> impl Iterator<Item = Vec<u32>> {
    std::iter::successors(Some(input), |input| Some(transform(&input)))
}

fn calc_offset(input: &[u32]) -> usize {
    let offset: String = input.iter().take(7).map(|n| n.to_string()).collect();
    offset.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ones() {
        let tests = &[
            (0_i32, 0),
            (1, 1),
            (-1, 1),
            (10, 0),
            (-10, 0),
            (11, 1),
            (-11, 1),
        ];
        for (input, expected) in tests {
            assert_eq!(&ones(*input), expected);
        }
    }

    #[test]
    fn test_parse_input() {
        let tests = &[("", vec![]), ("012", vec![0, 1, 2])];
        for (input, expected) in tests {
            assert_eq!(&parse_input(input), expected);
        }
    }

    #[test]
    fn test_pattern_iter() {
        let tests = &[
            (1, vec![0_i32, 1, 0, -1, 0, 1, 0, -1]),
            (
                2,
                vec![0_i32, 0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, -1],
            ),
        ];
        for (input, expected) in tests {
            let pat = pattern(*input);
            let pat: Vec<_> = pat.take(expected.len()).collect();
            assert_eq!(&pat, expected);
        }
    }

    #[test]
    fn test_transform() {
        let tests = &[
            ("80871224585914546619083218645595", "24176176"),
            (include_str!("day16.input"), "68764632"),
        ];
        for (input, expected) in tests {
            let input = parse_input(input);
            let input = transform_iter(input).skip(100).next().unwrap();
            let expected = parse_input(expected);
            assert_eq!(input[0..8].to_vec(), expected);
        }
    }

    #[test]
    fn test_calc_offset() {
        let input = "1234567";
        let input = parse_input(input);
        let offset = calc_offset(&input);
        assert_eq!(offset, 1234567)
    }

    #[test]
    // FIXME: Each test takes hours
    #[ignore]
    fn test_part_2() {
        let tests = &[
            ("03036732577212944063491565474664", "84462026"),
            (include_str!("day16.input"), ""),
        ];
        for (input, expected) in tests {
            let input = parse_input(input);
            let input: Vec<_> = std::iter::repeat(input).take(10_000).flatten().collect();
            let input = transform_iter(input).skip(100).next().unwrap();
            let offset = calc_offset(&input);
            let expected = parse_input(expected);
            assert_eq!(input[offset..offset + 8].to_vec(), expected);
        }
    }
}
