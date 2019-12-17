// https://adventofcode.com/2019/day/7

use crate::day5;
use itertools::Itertools;

fn run_program(phases: &[i64], program: &[i64]) -> i64 {
    let mut output = 0;
    for phase in phases {
        let mut program = day5::Program::new(program.to_vec());
        output = *program.run(&[*phase, output]).last().unwrap();
    }
    output
}

fn run_program_recursive(phases: &[i64], program: &[i64]) -> i64 {
    let mut output = 0;
    let mut programs: Vec<_> = (0..phases.len())
        .map(|_| day5::Program::new(program.to_vec()))
        .collect();
    for (i, phase) in phases.iter().enumerate() {
        let outputs = programs[i].run(&[*phase, output]);
        output = *outputs.last().unwrap();
    }
    loop {
        for i in 0..programs.len() {
            let outputs = programs[i].run(&[output]);
            output = *outputs.last().unwrap();
            if programs[i].complete && i == programs.len() - 1 {
                return output;
            }
        }
    }
}

fn combinations(items: &[i64]) -> Vec<Vec<i64>> {
    items
        .iter()
        .cloned()
        .permutations(items.len())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let tests = &[(
            (
                "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0",
                "4,3,2,1,0",
            ),
            43210,
        )];
        for (input, expected) in tests {
            let (program, thruster_signal) = input;
            let program = day5::parse_input(program);
            let phases = day5::parse_input(thruster_signal);
            assert_eq!(&run_program(&phases, &program), expected);
        }

        let input = include_str!("day7.input").trim_end();
        let output = combinations(&(0..=4).collect::<Vec<i64>>())
            .iter()
            .map(|phases| {
                let program = day5::parse_input(input);
                run_program(&phases, &program)
            })
            .max();
        assert_eq!(output, Some(929800));
    }

    #[test]
    fn test_part_2() {
        let tests = &[
            (("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5", "9,8,7,6,5"), 139629729),
            (("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10", "9,7,8,5,6"), 18216),
        ];
        for (input, expected) in tests {
            let (program, thruster_signal) = input;
            let program = day5::parse_input(program);
            let phases = day5::parse_input(thruster_signal);
            assert_eq!(&run_program_recursive(&phases, &program), expected);
        }

        let input = include_str!("day7.input").trim_end();
        let output = combinations(&(5..=9).collect::<Vec<i64>>())
            .iter()
            .map(|phases| {
                let program = day5::parse_input(input);
                run_program_recursive(&phases, &program)
            })
            .max();
        assert_eq!(output, Some(15432220));
    }
}
