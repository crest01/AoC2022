use std::fs;

#[allow(non_snake_case)]
fn to_item(b: u8) -> u32 {
    let b = b as u32;
    let Z = 'Z' as u32;
    let A = 'A' as u32;
    let a = 'a' as u32;
    // 65: A, 90: Z
    // 97: a, 122: z
    if b <= Z {
        return (b - A) + 27;
    }
    b - a + 1 
}

fn compartments(s: &str) -> (u64, u64) {
    let mut first = 0u64;
    let mut second = 0u64;
    let size = s.len();
    for i in 0..(size/2) {
        let item = to_item(s.as_bytes()[i]);
        first |= 1 << (item-1);
    }
    for i in (size/2)..size {
        let item = to_item(s.as_bytes()[i]);
        second |= 1 << (item -1);
    }
    (first, second)
}


pub fn parse_input(filename:&str) -> Vec<(u64, u64)>
{
    let contents = fs::read_to_string(filename).unwrap();
    let mut rucksack = Vec::<(u64, u64)>::new();
    for line in contents.lines() {
        rucksack.push(compartments(line.trim()));
    }
    rucksack
}

