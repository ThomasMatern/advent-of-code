#![allow(unused_variables,unused_imports,dead_code, unused_mut)]

use nom::{IResult, Parser, character::complete::{self, one_of}, multi::{separated_list1, many1}, sequence::separated_pair};
use nom_supreme::tag::complete::tag;
use itertools::Itertools;

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

type Row = Vec<Spring>;
type Groups = Vec<u32>;

fn parse_row(input: &str) -> IResult<&str, (Row, Groups)> {
    use Spring::*;

    separated_pair(
        many1(
            one_of(".#?").map(|c| {
                match c {
                    '.' => Operational,
                    '#' => Damaged,
                    '?' => Unknown,
                    _ => unreachable!("Unreachable"),
                }
            })
        ),
        tag(" "),
        separated_list1(
            tag(","),
            complete::u32))
        .parse(input)
}

fn is_valid(row: &Row, groups: &Groups) -> bool {
    &(row.iter()
        .group_by(|&spring| spring == &Spring::Damaged))
        .into_iter()
        .filter_map(|(damaged, group)| {
            if damaged {    
                Some(group.collect::<Vec<_>>().len() as u32)
            } else {
                None
            }
        })
        .collect::<Vec<u32>>() == groups

}

fn recurse(row: &mut Row, idx: usize, groups: &Groups) -> usize {
    use Spring::*;
    let mut idx = idx;

    // find next unknown spring
    while idx < row.len() && row[idx] != Unknown {
        idx += 1;
    }

    if idx == row.len() {
        if is_valid(row, groups) {
            return 1;
        } else {
            return 0;
        }
    }

    let mut count: usize = 0;
    for test in [Operational, Damaged] {
        row[idx] = test;
        count += recurse(row, idx+1, groups);
    }
    row[idx] = Unknown;
    count
}

fn count_arrangements(row: &mut Row, groups: &Groups) -> usize {

    recurse(row, 0, groups)
}

pub fn process(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let (_, (mut row, groups)) = parse_row(line).unwrap();
            count_arrangements(&mut row, &groups)
        })
        .sum::<usize>().to_string()
}



#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process_1() {
        let input = "\
#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1";
        assert_eq!("6", process(input));
    }

    #[test]
    fn test_process_2() {
        let input = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!("21", process(input));
    }

    #[test]
    fn test_process_3() {
        assert_eq!("1", process("???.### 1,1,3"));
    }

}
