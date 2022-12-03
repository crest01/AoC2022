mod parse;


fn get_bit_set_index(value: u64) -> i32 {
    for i in 0..64 {
        let mask: u64 = 1 << i;
        if value & mask == mask {
            return i+1;
        }
    }
    return 0;
}

fn task1(input:&Vec<(u64, u64)>) {
    println!("Task 1:");
    let mut sum = 0;
    for (left, right) in input.iter() {
        let duplicated_item = get_bit_set_index(left & right);
        sum += duplicated_item;
    }
    println!("  Sum prios: {}", sum);
}

fn task2(input: &Vec<(u64, u64)>) {
    println!("Task 2:");
    let mut sum = 0;
    let mut group_items: u64 = 0;
    let mut elf_index = 0;
    for (left, right) in input.iter() {
        let elf_item_mask = left | right;
        group_items = match elf_index {
            0 => elf_item_mask,
            1|2 => group_items & elf_item_mask,
            _ => unreachable!("Group index error"),
        };
        if elf_index == 2 {
            sum += get_bit_set_index(group_items);
            elf_index = 0;
        }
        else {
            elf_index += 1;
        }
    }
    println!("  Sum prios: {}", sum);
}

fn main() {
    println!("========== Test =============");
    let test_input = parse::parse_input("test.txt");
    println!("parsed test input: {:?}", test_input);
    println!("----");

    task1(&test_input);
    task2(&test_input);
    println!("========== Input ============");
    let input = parse::parse_input("input.txt");
    println!("parsed input: {} rucksacks", input.len());
    println!("----");
    task1(&input);
    task2(&input);
}
