mod parse;

use parse::Tree;
use std::collections::HashSet;

trait Buffer<T> {
    fn allocate_data(&mut self, elements: usize);
    fn data(&self) -> &Vec<T>;
    fn data_mut(&mut self) -> &mut Vec<T>;
    fn size(&self) -> &(u32, u32);
    fn size_mut(&mut self) -> &mut (u32, u32);
    fn init(&mut self, size: (u32, u32)) {
        self.allocate_data((size.0 * size.1) as usize);
        *self.size_mut() = size;
    }
    fn to_index(&self, pos: (u32, u32)) -> usize {
        return (pos.1 * self.size().0 + pos.0) as usize;
    }
    fn print(&self);
}

trait DepthBuffer<T> {
    fn try_set_value(&mut self, pos: (u32, u32), v: T) -> bool;
}

trait PixelBuffer<T> {
    fn set_value(&mut self, pos: (u32, u32), v: T);
}

struct TreeDepthBuffer {
    data: Vec<f32>,
    size: (u32, u32),
}

impl Buffer<f32> for TreeDepthBuffer {
    fn allocate_data(&mut self, elements: usize) {
        self.data = vec![f32::MAX; elements];
    }
    fn data(&self) -> &Vec<f32> {
        &self.data
    }
    fn data_mut(&mut self) -> &mut Vec<f32> {
        &mut self.data
    }
    fn size(&self) -> &(u32, u32) {
        &self.size
    }
    fn size_mut(&mut self) -> &mut (u32, u32) {
        &mut self.size
    }
    fn print(&self) {
        let size = self.size();
        let data = self.data();
        for y in 0..size.1 {
            print!("|");
            for x in 0..size.0 {
                let index = self.to_index((x, y));
                if data[index] == f32::MAX {
                    print!("    |",);
                } else {
                    print!("{:.2}|", data[index]);
                }
            }
            print!("\n");
        }
    }
}

impl DepthBuffer<f32> for TreeDepthBuffer {
    fn try_set_value(&mut self, pos: (u32, u32), v: f32) -> bool {
        let index = self.to_index(pos);
        let data = self.data_mut();
        let ele = &mut data[index];
        if *ele >= v {
            *ele = v;
            return true;
        }
        false
    }
}

struct TreeIdBuffer {
    data: Vec<u32>,
    size: (u32, u32),
}

impl Buffer<u32> for TreeIdBuffer {
    fn allocate_data(&mut self, elements: usize) {
        self.data = vec![u32::MAX; elements];
    }
    fn data(&self) -> &Vec<u32> {
        &self.data
    }
    fn data_mut(&mut self) -> &mut Vec<u32> {
        &mut self.data
    }
    fn size(&self) -> &(u32, u32) {
        &self.size
    }
    fn size_mut(&mut self) -> &mut (u32, u32) {
        &mut self.size
    }
    fn print(&self) {
        let size = self.size();
        let data = self.data();
        for y in 0..size.1 {
            print!("|");
            for x in 0..size.0 {
                let index = self.to_index((x, y));
                if data[index] == u32::MAX {
                    print!("   |");
                } else {
                    print!("{:>3}|", data[index]);
                }
            }
            print!("\n");
        }
    }
}

impl PixelBuffer<u32> for TreeIdBuffer {
    fn set_value(&mut self, pos: (u32, u32), v: u32) {
        let index = self.to_index(pos);
        let data = self.data_mut();
        let ele = &mut data[index];
        *ele = v;
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn splat(
    depth_buffer: &mut TreeDepthBuffer,
    pixel_buffer: &mut TreeIdBuffer,
    field_size: (u32, u32),
    direction: Direction,
    tree_id: u32,
    tree: &Tree,
) {
    let buffer_x = match direction {
        Direction::Left | Direction::Right => tree.pos.y as u32,
        Direction::Up | Direction::Down => tree.pos.x as u32,
    };
    let depth = match direction {
        Direction::Left => tree.pos.x as f32,
        Direction::Right => field_size.0 as f32 - tree.pos.x as f32,
        Direction::Down => field_size.1 as f32 - tree.pos.y as f32,
        Direction::Up => tree.pos.y as f32,
    };

    // tree height 0 is also visible
    for height in 0..(tree.height + 1.0f32) as u32 {
        let buffer_y = height;
        if depth_buffer.try_set_value((buffer_x, buffer_y), depth) {
            pixel_buffer.set_value((buffer_x, buffer_y), tree_id);
        }
    }
}

fn splat_trees(
    trees: &Vec<Tree>,
    field_size: (u32, u32),
) -> ([TreeIdBuffer; 4], [TreeDepthBuffer; 4], [Direction; 4]) {
    let mut id_buffers = [
            TreeIdBuffer {
                data: Vec::<u32>::new(),
                size: (0, 0),
            },
            TreeIdBuffer {
                data: Vec::<u32>::new(),
                size: (0, 0),
            },
            TreeIdBuffer {
                data: Vec::<u32>::new(),
                size: (0, 0),
            },
            TreeIdBuffer {
                data: Vec::<u32>::new(),
                size: (0, 0),
            },
    ];

    let mut depth_buffers = [
            TreeDepthBuffer {
                data: Vec::<f32>::new(),
                size: (0, 0),
            },
            TreeDepthBuffer {
                data: Vec::<f32>::new(),
                size: (0, 0),
            },
            TreeDepthBuffer {
                data: Vec::<f32>::new(),
                size: (0, 0),
            },
            TreeDepthBuffer {
                data: Vec::<f32>::new(),
                size: (0, 0),
            },
    ];

    let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

    let buffer_size_horizontal = (field_size.0, 10);
    let buffer_size_vertical = (field_size.1, 10);
    for i in 0..4 {
        match directions[i] {
            Direction::Up | Direction::Down => {
                id_buffers[i].init(buffer_size_horizontal);
                depth_buffers[i].init(buffer_size_horizontal);
            }
            Direction::Left | Direction::Right => {
                id_buffers[i].init(buffer_size_vertical);
                depth_buffers[i].init(buffer_size_vertical);
            }
        }
    }

    for i in 0..4 {
        for (tree_index, tree) in trees.iter().enumerate() {
            splat(
                &mut depth_buffers[i],
                &mut id_buffers[i],
                field_size,
                directions[i],
                tree_index as u32,
                tree,
            );
        }
        // println!("id-buffer for direction {:?}:", depth_buffers[i].1);
        // id_buffers[i].0.print();
        // println!("depth-buffer:");
        // depth_buffers[i].0.print();
    }
    (id_buffers, depth_buffers, directions)
}

fn task1(trees: &Vec<Tree>, field_size: (u32, u32)) {
    println!("Task 1:");
    let (id_buffers, _, _) = splat_trees(&trees, field_size);
    let tree_ids: HashSet<&u32> = HashSet::from_iter(
        id_buffers[0].data().iter().chain(
            id_buffers[1].data().iter().chain(
                id_buffers[2]
                    .data()
                    .iter()
                    .chain(id_buffers[3].data().iter()),
            ),
        ),
    );
    println!("Visible trees: {}", tree_ids.len() - 1);
}

fn get_tree_height(trees: &Vec<Tree>, pos: &(i32, i32), field_size: &(u32, u32)) -> u32 {
    let index = (pos.1 * field_size.0 as i32 + pos.0) as usize;
    trees[index].height as u32
}

fn check_tree_height(trees: &Vec<Tree>, start_pos: &(u32, u32), direction: &(i32, i32), field_size: &(u32, u32)) -> u32 {
    let mut test_pos = (start_pos.0 as i32, start_pos.1 as i32);
    let start_height = get_tree_height(&trees, &test_pos, field_size);
    let mut steps = 0u32;
    test_pos.0 += direction.0;
    test_pos.1 += direction.1;
    while test_pos.0 >= 0 && test_pos.0 < field_size.0 as i32 && test_pos.1 >= 0 && test_pos.1 < field_size.1 as i32 {
        steps += 1;
        let test_height = get_tree_height(&trees, &test_pos, &field_size);
        if test_height >= start_height {
            break;
        }
        test_pos.0 += direction.0;
        test_pos.1 += direction.1;
    }
    steps
}

fn task2(trees: &Vec<Tree>, field_size: (u32, u32)) {
    println!("Task 2:");
    let mut max_distance = Vec::<u32>::new();

    for tree in trees.iter() {
        if tree.pos.x == 0.0f32 || tree.pos.y == 0.0f32 {
            max_distance.push(0);
        }
        else {
            max_distance.push(1);
        }
    }

    for tree in trees.iter() {
        let tree_index = (tree.pos.y as u32 * field_size.0 + tree.pos.x as u32) as usize;
        if max_distance[tree_index] == 0 {
            continue;
        }
        let start_pos = (tree.pos.x as u32, tree.pos.y as u32);
        let visible_up = check_tree_height(&trees, &start_pos, &(0, -1), &field_size);
        let visible_down = check_tree_height(&trees, &start_pos, &(0, 1), &field_size);
        let visible_left = check_tree_height(&trees, &start_pos, &(-1, 0), &field_size);
        let visible_right = check_tree_height(&trees, &start_pos, &(1, 0), &field_size);
        max_distance[tree_index] = visible_up * visible_down * visible_left * visible_right;
    }
    println!("Max scenic score: {}", max_distance.iter().max().unwrap());

}

fn main() {
    println!("========== Test =============");
    let (trees, field_size) = parse::parse_input("test.txt");
    println!(
        "parsed test input: size, {:?}, trees, {:?}",
        field_size, trees
    );
    println!("----");

    task1(&trees, field_size);
    task2(&trees, field_size);
    println!("========== Input ============");
    let (trees, field_size) = parse::parse_input("input.txt");
    println!("parsed input: {}x{} trees", field_size.0, field_size.1);
    println!("----");
    task1(&trees, field_size);
    task2(&trees, field_size);
}
