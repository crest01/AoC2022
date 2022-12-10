mod parse;
use parse::Instruction;

fn task1(instructions: &Vec<Instruction>) {
    println!("Task 1:");
    let mut register = 1i32;
    let mut cycle = 0i32;
    let mut check_cycle = 20i32;
    let mut register_sum = 0i32;
    for instruction in instructions.iter() {
        let (steps, next_register) = match instruction.op {
            parse::Operation::Noop => (1, register),
            parse::Operation::Addx => (2, register + instruction.value)
        };
        cycle += steps;
        if cycle >= check_cycle {
            let value = check_cycle * register;
            println!("Cycle {}, Register {}, value {}", check_cycle, register, value);
            register_sum += value;
            check_cycle += 40;
        }
        register = next_register;
    }
    println!("  Sum: {}", register_sum);
}

fn task2(instructions: &Vec<Instruction>) {
    println!("Task 2:");
    let mut register = 1i32;
    let mut lines = Vec::<String>::new();
    let mut line = String::from("");
    
    for instruction in instructions.iter() {
        let (steps, next_register) = match instruction.op {
            parse::Operation::Noop => (1, register),
            parse::Operation::Addx => (2, register + instruction.value)
        };
        for _cycle in 0..steps {
            let pixel_pos = line.len() as i32;
            if pixel_pos >= register -1 && pixel_pos <= register +1 {
                line.push('#');
            }
            else {
                line.push('.');
            }
            if line.len() == 40 {
                lines.push(line);
                line = String::from("");
            }
        }
        register = next_register;
    }
    for line in lines.iter() {
        println!("{}", line.as_str());
    }
}

fn main() {
    println!("========== Test =============");
    let instructions = parse::parse_input("test.txt");
    println!(
        "parsed {} test instructions", instructions.len()
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
