fn task1(input:&Vec<u32>) {
    println!("Task 1:");
    let mut increased = 0;
    // windows(): iterator over all overlapping pairs of size 2 in the vector
    for window in input.windows(2) {
        if window[0] < window[1] {
            increased += 1;
        }
    }
    println!("  Increased: {}", increased);
}

fn task2(input:Vec<u32>) {
    println!("Task 2:");
    let mut increased = 0;
    let summed_input:Vec<u32> = input.windows(3).map(|x| x.iter().sum::<u32>()).collect();
    for window in summed_input.windows(2) {
        if window[0] < window[1] {
            increased += 1;
        }
    }
    println!("  Increased: {}", increased);
}

fn main() {
    println!("Test:");
    let input:Vec<u32> = include_str!("../test_input.txt").lines().map(|x| x.parse::<u32>().unwrap()).collect();
    task1(&input);
    task2(input);

    println!("Full Input:");
    let input:Vec<u32> = include_str!("../input.txt").lines().map(|x| x.parse::<u32>().unwrap()).collect();
    task1(&input);
    task2(input);
}
