// https://adventofcode.com/2019/day/16

use num::Integer;

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

fn transform_value<'a, T, U>(input: T, pattern: U) -> i32
where
    T: IntoIterator<Item = &'a u32>,
    U: IntoIterator<Item = i32>,
{
    let sum = input
        .into_iter()
        .zip(pattern)
        .map(|(i, j)| (*i as i32) * j)
        .sum();
    ones(sum)
}

fn transform(input: &[u32]) -> Vec<u32> {
    // "When applying the pattern, skip the very first value exactly once."
    (0..input.len())
        .map(|i| transform_value(input, pattern(i + 1).skip(1)) as u32)
        .collect()
}

fn transform_2(input: &[u32], times: usize) -> Vec<u32> {
    // length of the repeated input
    let total_len = times * input.len();
    let mut output = Vec::with_capacity(total_len);
    for i in 0..total_len {
        let i = i + 1;
        let pat_len = i * BASE_PATTERN.len();
        let lcm = input.len().lcm(&pat_len);
        let repeat = lcm / input.len();
        let repeat = std::cmp::min(repeat, total_len);
        println!(
            "value_num: {}, input: {}, pattern: {}, lcm: {}, total_len: {}, repeat: {}",
            i,
            input.len(),
            pat_len,
            lcm,
            total_len,
            repeat,
        );
        let input = std::iter::repeat(input.iter()).take(repeat).flatten();
        let value = transform_value(input, pattern(i + 1).skip(1));
        output.push(value as u32);
    }
    dbg!(output.len());
    output
}

fn transform_iter(input: Vec<u32>) -> impl Iterator<Item = Vec<u32>> {
    std::iter::successors(Some(input), |input| Some(transform(&input)))
}

fn transform_iter_2(input: Vec<u32>, times: usize) -> impl Iterator<Item = Vec<u32>> {
    std::iter::successors(Some(input), move |input| Some(transform_2(&input, times)))
}

fn calc_offset(input: &[u32]) -> usize {
    let offset: String = input.iter().take(7).map(|n| n.to_string()).collect();
    offset.parse().unwrap()
}

fn least_common_multiple(n1: usize, n2: usize) -> usize {
    n1.lcm(&n2)
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
            ("12345678", "01029498"),
            ("03036732577212944063491565474664", "84462026"),
            (include_str!("day16.input"), ""),
        ];
        for (input, expected) in tests {
            let input = parse_input(input);
            let input = transform_iter_2(input, 10).skip(100).next().unwrap();
            let offset = calc_offset(&input);
            let expected = parse_input(expected);

            assert_eq!(input[offset..offset + 8].to_vec(), expected);
        }
    }
}
