mod parse;
use parse::{Sensor, Beacon};
use std::{iter::zip, collections::HashSet};

fn manhattan_distance(pos: &(i32, i32), end: &(i32, i32)) -> i32 {
    i32::abs(pos.0 - end.0) + i32::abs(pos.1 - end.1)
}

fn can_contain_beacon(sensor: &Sensor, s_b_distance: i32, pos: &(i32, i32)) -> bool {
    manhattan_distance(&(sensor.x, sensor.y), pos) <= s_b_distance
}

fn task1(input: &Vec<(Sensor, Beacon)>, line_to_check: i32) {
    println!("Task 1:");

    let min_extends = input.iter().fold((i32::MAX, i32::MAX), |accu, ele| (accu.0.min(ele.0.x.min(ele.1.x)), accu.1.min(ele.0.y.min(ele.1.y))));
    let max_extends = input.iter().fold((i32::MIN, i32::MIN), |accu, ele| (accu.0.max(ele.0.x.max(ele.1.x)), accu.1.max(ele.0.y.max(ele.1.y))));

    let distances: Vec<i32> = input.iter().map(|(s, b)| manhattan_distance(&(s.x, s.y), &(b.x, b.y))).collect();

    println!("Extents: ({}, {}) - ({}, {})", min_extends.0, min_extends.1, max_extends.0, max_extends.1);

    let possible_sensors = zip(input, &distances).filter(|((sensor, _), distance)| i32::abs(sensor.y - line_to_check) <= **distance).collect::<Vec<(&(Sensor, Beacon), &i32)>>();
    let mut impossible = 0;
    for x in min_extends.0..max_extends.0 {
        let mut covered = false;
        let mut is_beacon = false;
        let mut is_sensor = false;
        for ((sensor, beacon), distance) in possible_sensors.iter() {
            if beacon.x == x && beacon.y == line_to_check {
                is_beacon = true;
            }
            if sensor.x == x && sensor.y == line_to_check {
                is_sensor = true;
            }
            if can_contain_beacon(&sensor, **distance, &(x, line_to_check)) {
                covered = true;
            }
        }
        if covered && !is_beacon && !is_sensor {
            impossible +=1;
        }
    }
    println!("Impossible locations in line {}: {}", line_to_check, impossible);
}

fn line(start: (i32, i32), end: (i32, i32)) -> Vec<(i32, i32)> {
    let dx = if start.0 < end.0 { 1 } else { -1 };
    let dy = if start.1 < end.1 { 1 } else { -1 };
    let mut result = Vec::<(i32, i32)>::new();
    let mut cursor_x = start.0;
    let mut cursor_y = start.1;
    loop {
        if cursor_x == end.0 && cursor_y == end.1{
            break;
        }
        result.push((cursor_x, cursor_y));
        cursor_x += dx;
        cursor_y += dy;
    }
    result
}

fn get_border_points(center: (i32, i32), distance: i32) -> Vec<(i32, i32)> {
    let mut result = Vec::<(i32, i32)>::new();

    let left = (center.0 - distance, center.1);
    let right = (center.0 + distance, center.1);
    let top = (center.0, center.1 + distance);
    let bottom = (center.0, center.1 - distance);

    result.append(&mut line(left, top));
    result.append(&mut line(top, right));
    result.append(&mut line(right, bottom));
    result.append(&mut line(bottom, left));
    result
}

fn task2(input: &Vec<(Sensor, Beacon)>, max_extents: (i32, i32)) {
    println!("Task 2:");

    let distances: Vec<i32> = input.iter().map(|(s, b)| manhattan_distance(&(s.x, s.y), &(b.x, b.y))).collect();

    let min_extents = (0, 0);
    println!("Extents: ({:?}) - ({:?})", min_extents, max_extents);

    let mut points_to_test = HashSet::<(i32, i32)>::new(); 

    for ((sensor, _), distance) in zip(input, &distances) {
        let border_points = get_border_points((sensor.x, sensor.y), *distance+1);
        for p in border_points.iter() {
            if p.0 < 0 || p.0 > max_extents.0 {
                continue;
            }
            if p.1 < 0 || p.1 > max_extents.1 {
                continue;
            }
            points_to_test.insert(*p);
        }
        println!("{} possible points", points_to_test.len());
    }
    for (idx, p) in points_to_test.iter().enumerate() {
        let possible_sensors = zip(input, &distances).any(|((sensor, _), distance)| manhattan_distance(&(sensor.x, sensor.y), p) <= *distance);
        if possible_sensors == false {
            println!("Target coords ({:?})", p);
            println!("Tuning frequency: {}", p.0 as i64 * 4000000i64 + p.1 as i64);
            break;
        }
        if idx % 1000000 == 0 {
            println!("{}% done", (100.0f32 / points_to_test.len() as f32) * idx as f32 );
        }
    }

}

fn main() {
    println!("========== Test =============");
    let input = parse::parse_input("test.txt");
    println!("parsed input {:?}", input);
    println!("----");

    task1(&input, 10);
    task2(&input, (20, 20));
    println!("========== Input ============");
    let input = parse::parse_input("input.txt");
    println!("parsed {} lines", input.len());
    println!("----");
    task1(&input, 2000000);
    task2(&input, (4000000, 4000000));
}
