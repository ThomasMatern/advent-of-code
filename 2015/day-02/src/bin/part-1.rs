#![allow(unused_variables,unused_imports,dead_code, unused_mut)]

use nom::{IResult, Parser, character::complete::{line_ending, digit1}, multi::separated_list1, sequence::tuple};
use nom_supreme::{tag::complete::tag, ParserExt};


fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

fn parse(i: &str) -> IResult<&str, Vec<(usize, usize, usize)>> {
    separated_list1(
        line_ending,
        tuple((
            digit1.map(|d: &str| d.parse().unwrap()).terminated(tag("x")),
            digit1.map(|d: &str| d.parse().unwrap()).terminated(tag("x")),
            digit1.map(|d: &str| d.parse().unwrap())))).parse(i)
}

pub fn process(input: &str) -> String {
    let (_, list) = parse(input).unwrap();

    list.iter().map(|(x, y, z)| {
        let s1 = x*y;
        let s2 = x*z;
        let s3 = y*z;

        2*s1 + 2*s2 + 2*s3 + s1.min(s2.min(s3))
    }).sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        assert_eq!("58", process("2x3x4"));
        assert_eq!("43", process("1x1x10"));
    }
}
