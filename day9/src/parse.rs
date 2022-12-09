extern crate failure;
use std::fs;


#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
pub struct Instruction {
    pub direction: Direction,
    pub size: u32,
}

pub fn parse_input(filename:&str) -> Vec<Instruction> {
    let content = fs::read_to_string(filename).unwrap();
    let mut instructions = Vec::<Instruction>::new();
    for line in content.lines() {
        let mut eles = line.split_ascii_whitespace().take(2);
        let direction = match eles.next().unwrap() {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("Unknown direction")
        };
        let steps = eles.next().unwrap().parse::<u32>().unwrap();
        instructions.push(Instruction{direction:direction, size:steps
        });
    }
    instructions
}

