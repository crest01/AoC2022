extern crate nom;
use std::fs;
use nom::multi::separated_list1;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::char;
use nom::combinator::map;
use nom::branch::alt;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::sequence::terminated;

// nom parser, probably way to complicated, but it might come in useful in the future

// parse string as integer
fn a(s: &str) -> IResult<&str, u32> {
    map(char('A'), |_|1)(s)
}

fn b(s: &str) -> IResult<&str, u32> {
    map(char('B'), |_|2)(s)
}

fn c(s: &str) -> IResult<&str, u32> {
    map(char('C'), |_|3)(s)
}

fn x(s: &str) -> IResult<&str, u32> {
    map(char('X'), |_|1)(s)
}

fn y(s: &str) -> IResult<&str, u32> {
    map(char('Y'), |_|2)(s)
}

fn z(s: &str) -> IResult<&str, u32> {
    map(char('Z'), |_|3)(s)
}

fn play(s: &str) -> IResult<&str, u32> {
    alt((a, b, c, x, y, z))(s)
}

fn game(s: &str) -> IResult<&str, (u32, u32)>  {
    separated_pair(play, char(' '), play)(s)
}

fn games(s: &str) -> IResult<&str, Vec<(u32, u32)>> {
    terminated(separated_list1(line_ending, game), multispace0)(s)
}

pub fn parse_input(filename:&str) -> Vec<(u32, u32)>
{
    let contents = fs::read_to_string(filename).unwrap();
    let (remaining, result) = games(contents.as_str()).unwrap();
    assert!(remaining.is_empty());
    result
}

