pub fn parse_input(s: &str) -> Vec<i64> {
    s.trim_end()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

#[derive(Debug, PartialEq)]
enum Opcode {
    Add,
    Mult,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
    AdjRelBase,
}

impl Opcode {
    fn from_u8(i: u8) -> Self {
        match i {
            1 => Self::Add,
            2 => Self::Mult,
            3 => Self::Input,
            4 => Self::Output,
            5 => Self::JumpIfTrue,
            6 => Self::JumpIfFalse,
            7 => Self::LessThan,
            8 => Self::Equals,
            9 => Self::AdjRelBase,
            99 => Self::Halt,
            _ => panic!("invalid opcode: {}", i),
        }
    }
}

#[derive(Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl ParameterMode {
    fn from_u8(i: u8) -> Self {
        match i {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
            _ => panic!("invalid parameter: {}", i),
        }
    }
}

fn parse_instruction(s: &str) -> (Opcode, Vec<ParameterMode>) {
    if s.len() < 2 {
        return (Opcode::from_u8(s.parse::<u8>().unwrap()), vec![]);
    }
    let (params, opcode) = s.split_at(s.len() - 2);
    let opcode = Opcode::from_u8(opcode.parse::<u8>().unwrap());
    let params: Vec<_> = params
        .chars()
        .rev()
        .map(|c| ParameterMode::from_u8(c.to_digit(10).unwrap() as u8))
        .collect();
    (opcode, params)
}

#[derive(Debug)]
pub struct Program {
    program: Vec<i64>,
    position: usize,
    relative_base: i64,
    pub complete: bool,
}

impl Program {
    pub fn new(program: Vec<i64>) -> Self {
        let position = 0;
        let relative_base = 0;
        let complete = false;
        Program {
            program,
            position,
            relative_base,
            complete,
        }
    }

    fn get(&self, pos: usize) -> i64 {
        self.program.get(pos).cloned().unwrap_or(0)
    }

    fn put(&mut self, pos: usize, val: i64) {
        if pos >= self.program.len() {
            self.program.resize(pos + 1, 0);
        }
        self.program[pos] = val;
    }

    fn get_value(&self, pos: usize, mode: &ParameterMode) -> i64 {
        match mode {
            ParameterMode::Position => self.get(self.get(pos) as usize),
            ParameterMode::Immediate => self.get(pos),
            ParameterMode::Relative => self.get((self.get(pos) + self.relative_base) as usize),
        }
    }

    pub fn run(&mut self, inputs: &[i64]) -> Vec<i64> {
        let mut inputs = inputs.iter();
        let mut outputs: Vec<i64> = Vec::new();
        loop {
            // dbg!(&self);
            dbg!(
                &outputs,
                &self.position,
                &self.program[self.position..]
                    .iter()
                    .take(5)
                    .collect::<Vec<_>>(),
            );
            let instruction = self.program[self.position].to_string();
            match parse_instruction(&instruction) {
                (Opcode::Add, param_modes) => {
                    let a = self.get_value(
                        self.position + 1,
                        param_modes.get(0).unwrap_or(&ParameterMode::Position),
                    );
                    let b = self.get_value(
                        self.position + 2,
                        param_modes.get(1).unwrap_or(&ParameterMode::Position),
                    );
                    let target = self.program[self.position + 3] as usize;
                    self.put(target, a + b);
                    self.position += 4
                }
                (Opcode::Mult, param_modes) => {
                    let a = self.get_value(
                        self.position + 1,
                        param_modes.get(0).unwrap_or(&ParameterMode::Position),
                    );
                    let b = self.get_value(
                        self.position + 2,
                        param_modes.get(1).unwrap_or(&ParameterMode::Position),
                    );
                    let target = self.program[self.position + 3] as usize;
                    self.put(target, a * b);
                    self.position += 4
                }
                (Opcode::Input, param_modes) => {
                    if let Some(input) = inputs.next() {
                        let a = self.get_value(
                            self.position + 1,
                            param_modes.get(0).unwrap_or(&ParameterMode::Immediate),
                        ) as usize;
                        dbg!(&input, &param_modes, &a);
                        self.put(a, *input);
                        self.position += 2;
                    } else {
                        println!("not enough outputs");
                        return outputs;
                    }
                }
                (Opcode::Output, param_modes) => {
                    let a = self.get_value(
                        self.position + 1,
                        param_modes.get(0).unwrap_or(&ParameterMode::Position),
                    );
                    self.position += 2;
                    outputs.push(a);
                }
                (Opcode::JumpIfTrue, param_modes) => {
                    let a = self.get_value(
                        self.position + 1,
                        param_modes.get(0).unwrap_or(&ParameterMode::Position),
                    );
                    if a != 0 {
                        let b = self.get_value(
                            self.position + 2,
                            param_modes.get(1).unwrap_or(&ParameterMode::Position),
                        );
                        self.position = b as usize;
                    } else {
                        self.position += 3;
                    }
                }
                (Opcode::JumpIfFalse, param_modes) => {
                    let a = self.get_value(
                        self.position + 1,
                        param_modes.get(0).unwrap_or(&ParameterMode::Position),
                    );
                    if a == 0 {
                        let b = self.get_value(
                            self.position + 2,
                            param_modes.get(1).unwrap_or(&ParameterMode::Position),
                        );
                        self.position = b as usize;
                    } else {
                        self.position += 3;
                    }
                }
                (Opcode::LessThan, param_modes) => {
                    let a = self.get_value(
                        self.position + 1,
                        param_modes.get(0).unwrap_or(&ParameterMode::Position),
                    );
                    let b = self.get_value(
                        self.position + 2,
                        param_modes.get(1).unwrap_or(&ParameterMode::Position),
                    );
                    let target = self.program[self.position + 3] as usize;
                    self.put(target, if a < b { 1 } else { 0 });
                    self.position += 4;
                }
                (Opcode::Equals, param_modes) => {
                    let a = self.get_value(
                        self.position + 1,
                        param_modes.get(0).unwrap_or(&ParameterMode::Position),
                    );
                    let b = self.get_value(
                        self.position + 2,
                        param_modes.get(1).unwrap_or(&ParameterMode::Position),
                    );
                    let target = self.program[self.position + 3] as usize;
                    self.put(target, if a == b { 1 } else { 0 });
                    self.position += 4;
                }
                (Opcode::AdjRelBase, param_modes) => {
                    let a = self.get_value(
                        self.position + 1,
                        param_modes.get(0).unwrap_or(&ParameterMode::Position),
                    );
                    self.relative_base += a;
                    self.position += 2;
                }
                (Opcode::Halt, _param_modes) => {
                    self.complete = true;
                    return outputs;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse_input() {
        assert_eq!(parse_input("4,3,2,1,0"), &[4, 3, 2, 1, 0]);
    }

    fn my_input() -> Vec<i64> {
        let input = include_str!("day5.input");
        parse_input(input)
    }

    #[test]
    fn test_parse_instruction() {
        let tests = &[
            (
                "1002",
                (
                    Opcode::Mult,
                    vec![ParameterMode::Position, ParameterMode::Immediate],
                ),
            ),
            (
                "1101",
                (
                    Opcode::Add,
                    vec![ParameterMode::Immediate, ParameterMode::Immediate],
                ),
            ),
            ("101", (Opcode::Add, vec![ParameterMode::Immediate])),
            ("1", (Opcode::Add, vec![])),
        ];
        for (input, expected) in tests {
            assert_eq!(&parse_instruction(input), expected);
        }
    }

    #[test]
    fn test_run_program_part_1() {
        let input = my_input();
        let mut program = Program::new(input);
        assert_eq!(program.run(&[1]).last().cloned(), Some(5346030 as i64));
    }

    #[test]
    fn test_run_program_part_2() {
        let input = my_input();
        let mut program = Program::new(input);
        assert_eq!(program.run(&[5]).last().cloned(), Some(513116 as i64));
    }
}
