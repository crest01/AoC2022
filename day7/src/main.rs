mod parse;
use std::{collections::HashMap, thread::current};


fn get_directory_sizes(files: &Vec<(String, String, u32)>) -> HashMap::<String, u32> {
    let mut sizes = HashMap::<String, u32>::new();

    for (directory, _, size) in files.iter() {
        let mut path = directory.split("/").collect::<Vec<&str>>();
        while !path.is_empty() {
            let current_path = path.join("/");
            let directory_size = sizes.entry(current_path).or_insert(0);
            *directory_size += size;
            path.pop();
        }
    }
    sizes
}

fn task1(lines: &Vec<(String, String, u32)>) {
    println!("Task 1:");

    let sizes = get_directory_sizes(lines);
    let mut sum_sizes = 0;
    for (_, size) in sizes.iter() {
        if *size <= 100000 {
            sum_sizes += *size;
        }
    }
    println!("Sum (size < 100000): {}", sum_sizes);
}

fn task2(lines: &Vec<(String, String, u32)>) {
    println!("Task 2:");
    let sizes = get_directory_sizes(lines);

    let free_space = 70000000 - sizes.get("$").unwrap();
    let needed = 30000000 - free_space;
    let gb = 1f32 / (1024f32 * 1024f32 * 1024f32);
    println!("Free: {}, needed: {}", free_space as f32 * gb, needed as f32 * gb);

    let mut size_to_delete = u32::MAX;
    for (entry, size) in sizes.iter() {
        if *size >= needed {
            size_to_delete = size_to_delete.min(*size);
            println!("Candidate: {}, size {}", entry, *size as f32 * gb);
        }
    }
    println!("Size of smallest large directory: : {}", size_to_delete);

}

fn main() {
    println!("========== Test =============");
    let files = parse::parse_input("test.txt");
    println!("parsed test input: {:?}", files);
    println!("----");

    task1(&files);
    task2(&files);
    println!("========== Input ============");
    let files = parse::parse_input("input.txt");
    println!("parsed input: {} files", files.len());
    println!("----");
    task1(&files);
    task2(&files);
}
