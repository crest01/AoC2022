use std::fs;

fn parse_container_line(s: &str) -> Vec<char> {
    let mut result = Vec::<char>::new();
    for entry in s.as_bytes().chunks(4) {
        result.push(char::from(entry[1]));
    }
    return result
}

fn parse_command(s: &str) -> (i32, i32, i32) {
    let items = s.split(" ").collect::<Vec<&str>>();
    return (items[1].parse().unwrap(), items[3].parse().unwrap(), items[5].parse().unwrap());
}

fn assemble_containers(lines: &Vec<Vec<char>>, ids: &Vec<char>) -> Vec<Vec<char>> {
    let mut containers = Vec::<Vec<char>>::new();
    let mut container_idx;
    for _ in ids.iter() {
        containers.push(Vec::<char>::new());
    }
    for line in lines.iter() {
        container_idx = 0;
        for item in line.iter() {
            if *item != ' ' {
                containers[container_idx].push(*item);
            }
            container_idx += 1;
        }
    }
    containers
}

pub fn parse_input(filename:&str) -> (Vec<Vec<char>>, Vec<(i32, i32, i32)>)
{
    let content = fs::read_to_string(filename).unwrap();
    let mut state = 0;
    let mut container_lines = Vec::<Vec<char>>::new();
    let mut containers = Vec::<Vec<char>>::new();
    let mut commands = Vec::<(i32, i32, i32)>::new();
    for line in content.lines() {
        if line.len() > 0 {
            if state == 0 {
                let res = parse_container_line(&line);
                container_lines.push(res);
            }
            else if state == 1 {
                let res = parse_command(&line);
                commands.push(res);
            }
        }
        else {
            if state == 0 {
                let container_ids = container_lines.pop().unwrap();
                containers = assemble_containers(&container_lines, &container_ids);
            }
            state += 1;
        } 
    } 
    (containers, commands)
}

