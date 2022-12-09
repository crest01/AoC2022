use std::fs;

#[derive(Debug)]
pub struct Float2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Tree {
    pub pos: Float2,
    pub height: f32,
}

pub fn parse_input(filename:&str) -> (Vec<Tree>, (u32, u32)) {
    let content = fs::read_to_string(filename).unwrap();
    let mut trees = Vec::<Tree>::new();
    let mut x = 0f32;
    let mut y = 0f32;
    let mut max_x = 0f32;
    let mut max_y = 0f32;
    for line in content.lines() {
        for ele in line.chars() {
            let height = (ele as u8 - 48) as f32;
            trees.push(Tree{pos: Float2{x, y}, height: height});
            x += 1.0f32;
            max_x = max_x.max(x);
        }
        y += 1.0f32;
        x = 0.0f32;
        max_y = max_y.max(y);
    }
    (trees, (max_x as u32, max_y as u32))
}

