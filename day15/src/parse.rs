use std::fs;

#[derive(Debug, Copy, Clone)]
pub struct Sensor {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct Beacon {
    pub x: i32,
    pub y: i32,
}

pub fn parse_input(filename: &str) -> Vec<(Sensor, Beacon)> {
    let content = fs::read_to_string(filename).unwrap();
    let coords: Vec<Vec<&str>> = content
        .lines()
        .map(|x| {
            x.split_ascii_whitespace()
                .filter(|x| x.starts_with("x") || x.starts_with("y"))
                .collect()
        })
        .collect();
    let mut result = Vec::<(Sensor, Beacon)>::new();
    for set in coords.iter() {
        let numbers: Vec<i32> = set
            .iter()
            .map(|x| {
                x.split(|x| x == '=' || x == ':' || x == ',')
                    .skip(1)
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap()
            })
            .collect();
        result.push((Sensor{x:numbers[0], y:numbers[1]}, Beacon{x:numbers[2], y:numbers[3]}));
    }
    result
}
