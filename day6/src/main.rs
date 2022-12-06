use std::collections::VecDeque;

mod parse;


fn task1(stacks: &Vec<Vec<u8>>) {
    println!("Task 1:");
}

fn task2(stacks: &Vec<Vec<u8>>) {
    println!("Task 2:");
}

fn main() {
    println!("========== Test =============");
    let lines = parse::parse_input("test.txt");
    println!("parsed test input: {:?}", lines);
    println!("----");

    task1(&lines);
    task2(&lines);
    // println!("========== Input ============");
    // let (stacks, commands) = parse::parse_input("input.txt");
    // println!("parsed input: {} stacks, {} commands", stacks.len(), commands.len());
    // println!("----");
    // task1(&stacks, &commands);
    // task2(&stacks, &commands);
}
