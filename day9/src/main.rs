mod parse;
use parse::Instruction;
use std::collections::HashSet;

trait Location {
    fn pos(&self) -> &(i32, i32);
    fn pos_mut(&mut self) -> &mut (i32, i32);
}

trait PathExtents {
    fn add_pos(&mut self, pos: &(i32, i32));
    fn get_extents(&self) -> ((i32, i32), (i32, i32));
}

trait UniquePath {
    fn add_pos(&mut self, pos: &(i32, i32));
    fn count_unique(&self) -> i32;
}


struct Head {
    p: (i32, i32),
    max: (i32, i32),
    min: (i32, i32)
}

impl Location for Head {
    fn pos(&self) -> &(i32, i32) {
        &self.p
    }
    fn pos_mut(&mut self) -> &mut (i32, i32) {
        &mut self.p
    }
}

impl PathExtents for Head {
    fn add_pos(&mut self, pos: &(i32, i32)) {
        self.max = (self.max.0.max(pos.0), self.max.1.max(pos.1));
        self.min = (self.min.0.min(pos.0), self.min.1.min(pos.1));
    }
    fn get_extents(&self) -> ((i32, i32), (i32, i32)) {
        let extend = (self.max.0 - self.min.0, self.max.1 - self.min.1);
        let offset = self.min;
        (extend, offset)
    }
}

struct Tail {
    p: (i32, i32),
    path: HashSet<(i32, i32)>
}

impl Location for Tail {
    fn pos(&self) -> &(i32, i32) {
        &self.p
    }
    fn pos_mut(&mut self) -> &mut (i32, i32) {
        &mut self.p
    }
}

impl UniquePath for Tail {
    fn add_pos(&mut self, pos: &(i32, i32)) {
        self.path.insert(*pos);
    }
    fn count_unique(&self) -> i32 {
        self.path.len() as i32
    }
}

fn step(head: &mut (impl Location + PathExtents), direction: parse::Direction) {
    *head.pos_mut() = match direction {
        parse::Direction::Up => (head.pos().0, head.pos().1 + 1),
        parse::Direction::Down => (head.pos().0, head.pos().1 - 1),
        parse::Direction::Left => (head.pos().0 - 1, head.pos().1),
        parse::Direction::Right => (head.pos().0 + 1, head.pos().1),
    };
    let current_pos = *head.pos();
    head.add_pos(&current_pos);
}

fn follow(head: &impl Location, tail: &mut (impl Location + UniquePath)) {
    let mut dx = head.pos().0 - tail.pos().0;
    let mut dy = head.pos().1 - tail.pos().1;

    if i32::abs(dx) <= 1 && i32::abs(dy) <= 1 {
        return;
    }
    if dx < 0 {
        dx = -1.max(dx);
    }
    else if dx > 0 {
        dx = 1.min(dx)
    }
    if dy < 0 {
        dy = -1.max(dy)
    }
    else if dy > 0 {
        dy = 1.min(dy)
    } 
    let new_pos = (tail.pos().0 + dx, tail.pos().1 + dy);
    *tail.pos_mut() = new_pos;
    tail.add_pos(&new_pos);
}

fn print_map(map: &Vec<Vec<char>>, size: (i32, i32)) {
    println!("-----");
    for col in 0..size.1 {
        for row in 0..size.0 {
            print!("{}", map[row as usize][col as usize]);
        }
        println!();
    }
}

fn task1(instructions: &Vec<Instruction>) {
    println!("Task 1:");

    let mut head= Head{ p: (0, 0), max: (0, 0), min: (0, 0)};
    let mut tail = Tail{p: (0, 0), path: HashSet::<(i32, i32)>::new()};
    let current_tail = *tail.pos();
    tail.add_pos(&current_tail);
    for instruction in instructions.iter() {
        for _ in 0..instruction.size {
            step(&mut head, instruction.direction);
            follow(&head, &mut tail);
        }
    }
    println!("Extents: {:?}", head.get_extents());
    println!("Unique Positions: {}", tail.count_unique());
}

fn task2(instructions: &Vec<Instruction>) {
    println!("Task 2:");
    let mut head= Head{ p: (0, 0), max: (0, 0), min: (0, 0)};
    let mut tail_0 = Tail{p: (0, 0), path: HashSet::<(i32, i32)>::new()};
    let mut tail_1 = Tail{p: (0, 0), path: HashSet::<(i32, i32)>::new()};
    let mut tail_2 = Tail{p: (0, 0), path: HashSet::<(i32, i32)>::new()};
    let mut tail_3 = Tail{p: (0, 0), path: HashSet::<(i32, i32)>::new()};
    let mut tail_4 = Tail{p: (0, 0), path: HashSet::<(i32, i32)>::new()};
    let mut tail_5 = Tail{p: (0, 0), path: HashSet::<(i32, i32)>::new()};
    let mut tail_6 = Tail{p: (0, 0), path: HashSet::<(i32, i32)>::new()};
    let mut tail_7 = Tail{p: (0, 0), path: HashSet::<(i32, i32)>::new()};
    let mut tail_8 = Tail{p: (0, 0), path: HashSet::<(i32, i32)>::new()};

    let current_pos = *head.pos();
    tail_0.add_pos(&current_pos);
    tail_1.add_pos(&current_pos);
    tail_2.add_pos(&current_pos);
    tail_3.add_pos(&current_pos);
    tail_4.add_pos(&current_pos);
    tail_5.add_pos(&current_pos);
    tail_6.add_pos(&current_pos);
    tail_7.add_pos(&current_pos);
    tail_8.add_pos(&current_pos);

    for instruction in instructions.iter() {
        for _ in 0..instruction.size {
            step(&mut head, instruction.direction);
            follow(&head, &mut tail_0);
            follow(&tail_0, &mut tail_1);
            follow(&tail_1, &mut tail_2);
            follow(&tail_2, &mut tail_3);
            follow(&tail_3, &mut tail_4);
            follow(&tail_4, &mut tail_5);
            follow(&tail_5, &mut tail_6);
            follow(&tail_6, &mut tail_7);
            follow(&tail_7, &mut tail_8);
        }
    }
    println!("Unique Positions of tail: {}", tail_8.count_unique());


}

fn main() {
    println!("========== Test =============");
    let instructions = parse::parse_input("test.txt");
    println!(
        "parsed test input: {:?}", instructions
    );
    println!("----");

    task1(&instructions);
    task2(&instructions);
    println!("========== Input ============");
    let instructions = parse::parse_input("input.txt");
    println!("parsed input: {} instructions", instructions.len());
    println!("----");
    task1(&instructions);
    task2(&instructions);
}
