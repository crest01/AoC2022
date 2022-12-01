extern crate nom;
use std::fs;
use nom::multi::separated_list1;
use nom::multi::count;
use nom::character::complete::line_ending;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::combinator::map_res;
use nom::IResult;
use nom::sequence::terminated;

// nom parser, probably way to complicated, but it might come in useful in the future

// parse string as integer
fn integer(s: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(s)
}

// parse newline separated list to Vector of integers
fn elf(s: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(line_ending, integer)(s)
}

// parse a nested list, each sub-list separated by two newlines
fn elves(s: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(count(line_ending, 2), elf)(s)
}

// parse a list of elves followed by a newline
fn input(s: &str) -> IResult<&str, Vec<Vec<u32>>> {
    terminated(elves, multispace0)(s)
}

fn parse_input(filename:&str) -> Vec<Vec<u32>>
{
    let contents = fs::read_to_string(filename).unwrap();
    let (remaining, result) = input(contents.as_str()).unwrap();
    assert!(remaining.is_empty());
    result
}

fn task1(input:&Vec<Vec<u32>>) {
    println!("Task 1:");
    let max = input.iter().map(|x| x.iter().sum::<u32>()).max().unwrap();
    println!("Max calories per elf: {}", max);
}

fn task2(input: &Vec<Vec<u32>>) {
    println!("Task 2:");
    let mut max = input.iter().map(|x| x.iter().sum::<u32>()).collect::<Vec<u32>>().to_owned();
    max.sort();
    let mut sum = 0;
    for i in 1..4 {
        sum += max[max.len()-i];
    }
    println!("top three elf calories: {:?}", sum);
}

fn main() {
    println!("Test:");
    let test_input = parse_input("test.txt");
    task1(&test_input);
    task2(&test_input);

    println!("Full Input:");
    let input = parse_input("input.txt");
    task1(&input);
    task2(&input);
}
