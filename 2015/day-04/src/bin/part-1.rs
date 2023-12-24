#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

use nom::{
    IResult,
    Parser,
    character::complete::{ alphanumeric1, line_ending, self, digit1 },
    multi::{ separated_list1, count },
    sequence::{ separated_pair, tuple },
};
use nom_supreme::{ tag::complete::tag, ParserExt };
use itertools::Itertools;
use rayon::{ prelude::*, iter::ParallelDrainFull };
use indicatif::{ ProgressIterator, ParallelProgressIterator };

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

pub fn process(input: &str) -> String {
    (0..)
        .find(|num| {
            let value = format!("{}{}", input, num);
            let hash = md5::compute(value);
            format!("{:x}", hash).starts_with("00000")
        })
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        assert_eq!("609043", process("abcdef"));
    }
}
