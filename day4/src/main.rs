mod parse;


fn contains_point(line:&(u32, u32), point:&u32) -> bool{
    line.0 <= *point && line.1 >= *point
}

fn task1(input:&Vec<((u32, u32), (u32, u32))>) {
    println!("Task 1:");
    let mut contained = 0;
    for (s1, s2) in input.iter()  {
        if contains_point(s1, &s2.0) && contains_point(s1, &s2.1) {
            contained += 1;
        }
        else if contains_point(s2, &s1.0) && contains_point(s2, &s1.1) {
            contained += 1;
        }
    }
    println!("  Contained: {}", contained)
}


fn task2(input: &Vec<((u32, u32), (u32, u32))>) {
    println!("Task 2:");
    let mut overlap = 0;
    for (s1, s2) in input.iter()  {
        if contains_point(s1, &s2.0) || contains_point(s1, &s2.1) {
            overlap += 1;
        }
        else if contains_point(s2, &s1.0) || contains_point(s2, &s1.1) {
            overlap += 1;
        }
    }
    println!("  Overlapping: {}", overlap)
}

fn main() {
    println!("========== Test =============");
    let test_input = parse::parse_input("test.txt");
    println!("parsed test input: {:?}", test_input);
    println!("----");

    task1(&test_input);
    task2(&test_input);
    println!("========== Input ============");
    let input = parse::parse_input("input.txt");
    println!("parsed input: {} lines", input.len());
    println!("----");
    task1(&input);
    task2(&input);
}
