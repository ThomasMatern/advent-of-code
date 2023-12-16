#![allow(unused_variables,unused_imports,dead_code, unused_mut)]

use nom::{IResult, Parser, character::complete::{alphanumeric1, line_ending, self, digit1}, multi::{separated_list1, count}, sequence::{separated_pair, tuple}};
use nom_supreme::{tag::complete::tag, ParserExt};
use itertools::Itertools;
use rayon::{prelude::*, iter::ParallelDrainFull};
use indicatif::{ProgressIterator, ParallelProgressIterator};


fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

fn parse(i: &str) -> IResult<&str, <>> {
    todo!()
}

pub fn process(input: &str) -> String {
    let (_, _) = parse(input).unwrap();
    todo()
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
