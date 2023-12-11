#![allow(unused_variables,unused_imports,dead_code)]

use nom::{IResult, Parser, character::complete::{alphanumeric1, line_ending}, multi::{separated_list1, count}, sequence::separated_pair};
use nom_supreme::{tag::complete::tag, ParserExt};

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

pub fn process(_input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "";
        assert_eq!("", process(input));
    }
}
