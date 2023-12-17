#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

use indicatif::{ParallelProgressIterator, ProgressIterator};
use itertools::Itertools;
use nom::{
    character::complete::{self, alphanumeric1, digit1, line_ending},
    multi::{count, separated_list1},
    sequence::{separated_pair, tuple},
    IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};
use rayon::{iter::ParallelDrainFull, prelude::*};

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

fn parse(i: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(line_ending, alphanumeric1).parse(i)
}

pub fn process(input: &str) -> String {
    let (_, lines) = parse(input).unwrap();

    lines
        .iter()
        .filter(|line| {
            line.chars()
                .zip(line.chars().skip(1))
                .any(|(a, b)| line.matches(&format!("{}{}", a, b)).count() >= 2)
        })
        .filter(|line| line.chars().zip(line.chars().skip(2)).any(|(a, b)| a == b))
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        assert_eq!("1", process("qjhvhtzxzqqjkmpb"));
        assert_eq!("1", process("xxyxx"));
        assert_eq!("0", process("uurcxstgmygtbstg"));
        assert_eq!("0", process("ieodomkazucvgmuy"));
    }
}
