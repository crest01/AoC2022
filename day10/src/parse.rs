use std::fs;


#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Noop,
    Addx,
}

#[derive(Debug)]
pub struct Instruction {
    pub op: Operation,
    pub value: i32,
}

pub fn parse_input(filename:&str) -> Vec<Instruction> {
    let content = fs::read_to_string(filename).unwrap();
    let mut instructions = Vec::<Instruction>::new();
    for line in content.lines() {
        let mut parts = line.split_ascii_whitespace();
        instructions.push(match parts.next().unwrap() {
            "noop" => Instruction{ op: Operation::Noop, value: 0},
            "addx" => Instruction{ op: Operation::Addx, value: parts.next().unwrap().parse().unwrap()},
            _ => panic!("Unknown instruction")
        });
    }
    instructions
}

