#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

use std::collections::BTreeMap;

use nom::{
    IResult,
    Parser,
    character::complete,
    multi::separated_list1,
    sequence::separated_pair,
    bytes::complete::is_a,
};
use nom_supreme::tag::complete::tag;
use itertools::Itertools;
use rayon::prelude::*;
use indicatif::ParallelProgressIterator;

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Ord, PartialOrd, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

type Row = Vec<Spring>;
type Groups = Vec<usize>;
type Cache<'a> = BTreeMap<(&'a [Spring], &'a [usize]), usize>;

fn parse_row(input: &str) -> IResult<&str, (Row, Groups)> {
    use Spring::*;

    separated_pair(
        is_a(".#?").map(|row|
            std::iter
                ::repeat(row)
                .take(5)
                .join("?")
                .chars()
                .map(|c| {
                    match c {
                        '.' => Operational,
                        '#' => Damaged,
                        '?' => Unknown,
                        _ => unreachable!("Unreachable"),
                    }
                })
                .collect_vec()
        ),
        tag(" "),
        separated_list1(
            tag(","),
            complete::u32.map(|g| g as usize)
        ).map(|groups| groups.repeat(5))
    ).parse(input)
}

fn count_arrangements<'a>(row: &'a [Spring], groups: &'a [usize], cache: &mut Cache<'a>) -> usize {
    use Spring::*;

    if row.is_empty() {
        // nothing to check
        if groups.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    if groups.is_empty() {
        // no groups left
        if row.contains(&Damaged) {
            return 0;
        } else {
            return 1;
        }
    }

    if let Some(count) = cache.get(&(row, groups)) {
        return *count;
    }

    let mut count = 0;
    if row[0] == Operational || row[0] == Unknown {
        count += count_arrangements(&row[1..], groups, cache);
    }
    if
        (row[0] == Damaged || row[0] == Unknown) &&
        row.len() >= groups[0] && // enough room for group
        !row[..groups[0]].contains(&Operational)
    {
        // space possibly valid for group

        if row.len() == groups[0] {
            // last group consumes input
            count += count_arrangements(&row[groups[0]..], &groups[1..], cache);
        } else if row[groups[0]] != Damaged {
            // terminated by not-damaged item
            count += count_arrangements(&row[groups[0] + 1..], &groups[1..], cache);
        }
    }

    cache.insert((row, groups), count);

    count
}

pub fn process(input: &str) -> String {
    input
        .lines()
        .collect_vec()
        .into_par_iter()
        .progress()
        .map(|line| {
            let (_, (row, groups)) = parse_row(line).unwrap();
            let mut cache: Cache = BTreeMap::new();
            count_arrangements(&row, &groups, &mut cache)
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process_2() {
        let input =
            "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!("525152", process(input));
    }

    #[test]
    fn test_process_3() {
        assert_eq!("16", process("????.#...#... 4,1,1"));
    }

    #[test]
    fn test_process_4() {
        assert_eq!("1", process("???.### 1,1,3"));
    }

    #[test]
    fn test_process_5() {
        assert_eq!("506250", process("?###???????? 3,2,1"));
    }
}
