// https://adventofcode.com/2019/day/2

const OPCODE_ADD: usize = 1;
const OPCODE_MULT: usize = 2;
const OPCODE_HALT: usize = 99;

fn run_program(mut input: Vec<usize>) -> Vec<usize> {
    let mut i = 0;
    while i < input.len() {
        let opcode = input[i];
        match opcode {
            OPCODE_ADD => {
                let a = input[i + 1];
                let b = input[i + 2];
                let target = input[i + 3];
                input[target] = input[a] + input[b];
            }
            OPCODE_MULT => {
                let a = input[i + 1];
                let b = input[i + 2];
                let target = input[i + 3];
                input[target] = input[a] * input[b];
            }
            OPCODE_HALT => {
                break;
            }
            _ => panic!("invalid opcode: {}", opcode),
        }
        i += 4;
    }

    input
}

fn find_verb_noun(mut input: Vec<usize>, output: usize) -> Option<(usize, usize)> {
    for verb in 0..=99 {
        for noun in 0..=99 {
            input[1] = verb;
            input[2] = noun;
            if run_program(input.clone())[0] == output {
                return Some((verb, noun));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_input(input: &str) -> Vec<usize> {
        input
            .trim_end()
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect()
    }

    fn my_input() -> Vec<usize> {
        let input = include_str!("day2.input");
        parse_input(input)
    }

    #[test]
    fn test_run_program() {
        let tests = &[("1,9,10,3,2,3,11,0,99,30,40,50", 3500), ("1,0,0,0,99", 2)];
        for (input, expected) in tests {
            let input = parse_input(input);
            assert_eq!(&run_program(input)[0], expected);
        }

        let mut input = my_input();
        // replace position 1 with the value 12 and replace position 2 with the
        // value 2
        input[1] = 12;
        input[2] = 2;
        assert_eq!(run_program(input)[0], 3760627);
    }

    #[test]
    fn test_find_verb_noun() {
        let input = my_input();
        assert_eq!(find_verb_noun(input, 3760627), Some((12, 2)));

        let input = my_input();
        let (verb, noun) = find_verb_noun(input, 19690720).unwrap();
        assert_eq!(100 * verb + noun, 7195);
    }
}
