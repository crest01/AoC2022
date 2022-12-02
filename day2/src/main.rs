#[macro_use]
extern crate simple_error;

mod parse;

use simple_error::SimpleError;

/*
R = 1
P = 2
S = 3
*/

fn game(op: &u32, me: &u32) -> Result<u32, SimpleError> {
    match op {
        1 => match me { // Rock
            1 => return Ok(4), // Rock,     draw:  3 + 1
            2 => return Ok(8), // Paper,    win:   6 + 2
            3 => return Ok(3), // Scissors, loose: 0 + 3
            _ => bail!("wrong input")
        },
        2 => match me { // Paper
            1 => return Ok(1), // Rock,     loose: 0 + 1
            2 => return Ok(5), // Paper,    draw:  3 + 2
            3 => return Ok(9), // Scissors, win:   6 + 3
            _ => bail!("wrong input")
        },
        3 => match me { // Scissors
            1 => return Ok(7), // Rock,     win:   6 + 1
            2 => return Ok(2), // Paper,    loose: 0 + 2
            3 => return Ok(6), // Scissors, draw:  3 + 3
            _ => bail!("wrong input")
        },
        _ => bail!("wrong input")
    }
}

/*
R = 1
P = 2
S = 3

1 = loose
2 = draw
3 = win

*/

fn get_response(op: &u32, me: &u32) -> Result<u32, SimpleError> {
    match op {
        1 => match me { // Rock
            1 => return Ok(3), // loose: Scissors
            2 => return Ok(1), // draw: Rock 
            3 => return Ok(2), // win: Paper
            _ => bail!("wrong input")
        },
        2 => match me { // Paper
            1 => return Ok(1), // loose: Rock
            2 => return Ok(2), // draw:  Paper
            3 => return Ok(3), // win:   Scissors
            _ => bail!("wrong input")
        },
        3 => match me { // Scissors
            1 => return Ok(2), // loose: Paper 
            2 => return Ok(3), // draw:  Scissors
            3 => return Ok(1), // win:   Rock
            _ => bail!("wrong input")
        },
        _ => bail!("wrong input")
    }
}



fn task1(input:&Vec<(u32, u32)>) {
    println!("Task 1:");
    let mut score = 0;
    for (op, me) in input {
        score += game(op, me).unwrap();
    }
    println!("  Score: {}", score);
}

fn task2(input: &Vec<(u32, u32)>) {
    println!("Task 2:");
    let mut score = 0;
    for (op, choice) in input {
        let me = get_response(op, choice).unwrap();
        let game_score = game(op, &me).unwrap();
        // println!("op: {}, choice: {}, me: {}, score: {}", op, choice, me, game_score);
        score += game_score;
    }
    println!("  Score: {}", score);
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
    println!("parsed test input: {} games", input.len());
    println!("----");
    task1(&input);
    task2(&input);
}
