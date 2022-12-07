use std::fs;

pub fn parse_input(filename:&str) -> Vec<(String, String, u32)> {
    let content = fs::read_to_string(filename).unwrap();
    let mut files = Vec::<(String, String, u32)>::new();
    let mut path = Vec::<&str>::new();
    for line in content.lines() {
        if line.starts_with("$ cd") {
            let new_directory = line.split_ascii_whitespace().take(3).collect::<Vec<&str>>()[2];
            match new_directory {
                ".." => {path.pop();},
                "/" => {path.clear(); path.push("$");},
                _ => path.push(new_directory)
            };
        }
        else if line.starts_with("$ ls") || line.starts_with("dir ") {
            continue;
        }
        else if line.len() > 0{
            let mut parts = line.split_ascii_whitespace().take(2);
            let size = parts.next().unwrap().parse::<u32>().unwrap();
            let name = parts.next().unwrap();
            files.push((String::from(path.join("/")), String::from(name), size));
        }
    }
    return files;
}

