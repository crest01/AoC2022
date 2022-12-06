mod parse;

fn find_uniques(line: &Vec<u8>, window_size: usize) -> usize {
    println!("Task 1:");
    for (index, window) in line.windows(window_size).enumerate() {
        let mut sum = 0u32;
        for item in window.iter() {
            sum |= 1 << (item - 1);
        }
        if sum.count_ones() == window_size as u32 {
            return index;
        }
    }
    return 0;
}

fn task1(lines: &Vec<Vec<u8>>) {
    println!("Task 1:");
    let window_size = 4;
    for (line_nr, line) in lines.iter().enumerate() {
        let unique = find_uniques(&line, window_size);
        println!("Line {}: {} chars", line_nr, unique + window_size);
    }
}

fn task2(lines: &Vec<Vec<u8>>) {
    println!("Task 2:");
    let window_size = 14;
    for (line_nr, line) in lines.iter().enumerate() {
        let unique = find_uniques(&line, window_size);
        println!("Line {}: {} chars", line_nr, unique + window_size);
    }
}

fn main() {
    println!("========== Test =============");
    let lines = parse::parse_input("test.txt");
    println!("parsed test input: {:?}", lines);
    println!("----");

    task1(&lines);
    task2(&lines);
    println!("========== Input ============");
    let lines = parse::parse_input("input.txt");
    println!("parsed input: {} chars", lines[0].len());
    println!("----");
    task1(&lines);
    task2(&lines);
}
