use std::fs;

pub fn parse_input(filename:&str) -> Vec<Vec<u8>>
{
    let content = fs::read_to_string(filename).unwrap();
    content.lines().map(|s| s.chars().map(|c| (c as u8) - 96).collect::<Vec<u8>>()).collect()
}

