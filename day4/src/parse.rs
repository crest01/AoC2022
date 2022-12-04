use std::fs;
use std::str::FromStr;

use nom::IResult;
use nom::character::complete::digit1;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::sequence::terminated;

fn number(s: &str) -> IResult<&str, u32> {
    map_res(digit1, u32::from_str)(s)
}

fn segment(s: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(number, char('-'), number)(s)
}

fn segment_pair(s: &str) -> IResult<&str, ((u32, u32), (u32, u32))> {
    separated_pair(segment, char(','), segment)(s)
}

fn segments(s: &str) -> IResult<&str, Vec<((u32, u32),(u32, u32))>> {
    separated_list1(line_ending, segment_pair)(s)
}

fn parse_file(s: &str) -> IResult<&str, Vec<((u32, u32), (u32, u32))>>{
    terminated(segments, multispace0)(s)
}

pub fn parse_input(filename:&str) -> Vec<((u32, u32), (u32, u32))>
{
    let contents = fs::read_to_string(filename).unwrap();
    let (remaining, result) = parse_file(contents.as_str()).unwrap();
    assert!(remaining.is_empty());
    result
}

