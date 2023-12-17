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
        .filter(|line| line.chars().filter(|&c| "aeiou".contains(c)).count() >= 3)
        .filter(|line| {
            line.chars()
                .zip(line.chars().skip(1))
                .find(|(a, b)| a == b)
                .is_some()
        })
        .filter(|line| !(["ab", "cd", "pq", "xy"].iter().any(|pt| line.contains(pt))))
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        assert_eq!("1", process("ugknbfddgicrmopn"));
        assert_eq!("1", process("aaa"));
        assert_eq!("0", process("jchzalrnumimnmhp"));
        assert_eq!("0", process("haegwjzuvuyypxyu"));
        assert_eq!("0", process("dvszwmarrgswjxmb"));
    }
}
