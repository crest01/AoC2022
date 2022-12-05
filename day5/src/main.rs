use std::collections::VecDeque;

mod parse;


fn task1(stacks: &Vec<Vec<char>>, commands: &Vec<(i32, i32, i32)>) {
    println!("Task 1:");
    let mut deques = Vec::<VecDeque<char>>::new();
    for stack in stacks.iter() {
        deques.push(VecDeque::from(stack.to_owned()));
    }
    for (count, from, to) in commands.iter() {
        for _ in 0..*count {
            let c = deques[(*from -1) as usize].pop_front().unwrap();
            deques[(*to -1) as usize].push_front(c);
        }
    }
    print!("  Top containers after move: ");
    for deque in deques.iter() {
        print!("{}", deque.front().unwrap());
    }
    print!("\n");
}

fn task2(stacks: &Vec<Vec<char>>, commands: &Vec<(i32, i32, i32)>) {
    println!("Task 2:");
    let mut deques = Vec::<VecDeque<char>>::new();
    for stack in stacks.iter() {
        deques.push(VecDeque::from(stack.to_owned()));
    }
    for (count, from, to) in commands.iter() {
        let mut crane = Vec::<char>::new();
        for _ in 0..*count {
            let c = deques[(*from -1) as usize].pop_front().unwrap();
            crane.push(c);
        }
        for _ in 0..*count {
            deques[(*to -1) as usize].push_front(crane.pop().unwrap());
        }
    }
    print!("  Top containers after move: ");
    for deque in deques.iter() {
        print!("{}", deque.front().unwrap());
    }
    print!("\n");

}

fn main() {
    println!("========== Test =============");
    let (test_stacks, test_commands) = parse::parse_input("test.txt");
    println!("parsed test input: {:?}, {:?}", test_stacks, test_commands);
    println!("----");

    task1(&test_stacks, &test_commands);
    task2(&test_stacks, &test_commands);
    println!("========== Input ============");
    let (stacks, commands) = parse::parse_input("input.txt");
    println!("parsed input: {} stacks, {} commands", stacks.len(), commands.len());
    println!("----");
    task1(&stacks, &commands);
    task2(&stacks, &commands);
}
