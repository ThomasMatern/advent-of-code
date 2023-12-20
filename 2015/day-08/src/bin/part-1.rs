#![allow(unused_variables,unused_imports,dead_code, unused_mut)]

use nom::{IResult, Parser, character::complete::{alphanumeric1, line_ending, self, digit1, anychar}, multi::{separated_list1, count, many1}, sequence::{separated_pair, tuple, pair}, branch::alt};
use nom_supreme::{tag::complete::tag, ParserExt};
use itertools::Itertools;
use rayon::{prelude::*, iter::ParallelDrainFull};
use indicatif::{ProgressIterator, ParallelProgressIterator};


fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

fn parse(i: &str) -> Vec<&str> {
    i.lines().collect_vec()
}

pub fn process(input: &str) -> String {
    let lines = parse(input);
    let mut total_escaped = 0;
    let mut total_payload = 0;
    for line in lines {
        total_escaped += line.len();
        let result: IResult<&str, Vec<usize>> = many1(
            alt((
                tag("\\\"").map(|_| 1),
                tag("\\\\").map(|_| 1),
                tag("\"").map(|_| 0),
                tag("\\x").terminated(pair(complete::anychar, complete::anychar)).map(|_| 1),
                anychar.map(|_| 1),
            ))
        ).parse(line);
        let (_, l) = result.unwrap();
        total_payload += l.iter().sum::<usize>();

    }

    (total_escaped-total_payload).to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = include_str!("./test-1.txt");
        assert_eq!("12", process(input));
    }
}
