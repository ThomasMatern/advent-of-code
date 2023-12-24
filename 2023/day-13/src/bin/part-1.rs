#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

use std::collections::HashMap;

use nom::{
    IResult,
    Parser,
    character::complete::{ line_ending, one_of },
    multi::{ separated_list1, many1 },
    sequence::pair,
};
use itertools::Itertools;
use rayon::prelude::*;
use indicatif::{ ProgressIterator, ParallelProgressIterator };

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

type Row = Vec<bool>;
type Pattern = Vec<Row>;

fn parse(i: &str) -> IResult<&str, Vec<Pattern>> {
    separated_list1(
        pair(line_ending, line_ending),
        separated_list1(
            line_ending,
            many1(
                one_of(".#").map(|ch| {
                    match ch {
                        '#' => true,
                        '.' => false,
                        _ => unreachable!(),
                    }
                })
            )
        )
    ).parse(i)
}

fn process_pattern_horizontal(pattern: &Pattern) -> Option<usize> {
    let mut groups: HashMap<&Row, Vec<usize>> = HashMap::new();

    pattern
        .iter()
        .enumerate()
        .for_each(|(idx, row)| {
            groups.entry(row).or_default().push(idx);
        });

    groups
        .iter()
        .flat_map(|(_, group)| {
            group
                .windows(2)
                .filter_map(|w| (w[0] + 1 == w[1]).then_some(w[0]))
                .collect_vec()
        })
        .find(|&c| {
            let max_pairs = (c + 1).min(pattern.len() - c - 1);
            (1..max_pairs).all(|idx| {
                groups
                    .get(&pattern[c - idx])
                    .unwrap()
                    .contains(&(c + idx + 1))
            })
        })
}

fn transpose_pattern(pattern: &Pattern) -> Pattern {
    let rows = pattern.len();
    let cols = pattern[0].len();

    (0..cols).map(|col| { (0..rows).map(|row| pattern[row][col]).collect() }).collect()
}

fn process_pattern(pattern: &Pattern) -> usize {
    let mut cnt =
        process_pattern_horizontal(pattern)
            .map(|x| x + 1)
            .unwrap_or(0) * 100;
    cnt += process_pattern_horizontal(&transpose_pattern(pattern))
        .map(|x| x + 1)
        .unwrap_or(0);
    cnt
}

pub fn process(i: &str) -> String {
    let (_, patterns) = parse(i).unwrap();
    let result = patterns
        .iter()
        .map(|pattern| { process_pattern(pattern) })
        .sum::<usize>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input =
            "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!("405", process(input));
    }
}
