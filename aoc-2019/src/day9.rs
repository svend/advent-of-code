#[cfg(test)]
mod tests {
    use crate::day5;

    #[test]
    fn test_examples() {
        // should output the large number in the middle
        let input = day5::parse_input("104,1125899906842624,99");
        let mut program = day5::Program::new(input.to_vec());
        assert_eq!(program.run(&[]), &[input[1]]);

        // should output a 16-digit number
        let input = day5::parse_input("1102,34915192,34915192,7,4,7,99,0");
        let mut program = day5::Program::new(input.to_vec());
        assert_eq!(program.run(&[])[0].to_string().chars().count(), 16);

        // takes no input and produces a copy of itself as output
        let input = day5::parse_input("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        let mut program = day5::Program::new(input.to_vec());
        assert_eq!(program.run(&[]), input);
    }

    #[test]
    fn test_my_input() {
        let input = day5::parse_input(include_str!("day9.input"));
        let mut program = day5::Program::new(input.to_vec());
        // Answer is not 203
        assert_eq!(program.run(&[1]), &[]);
    }
}
