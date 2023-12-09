#![allow(unused_variables, unused_imports, dead_code, unused_must_use)]

use nom::{IResult, branch::alt, sequence::tuple, character::complete::{digit1, space0, space1}, multi::separated_list0, bytes::complete::tag};
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

fn parse_card(i: &str) -> IResult<&str, u32> {
    let (i, _) = tuple((tag("Card"), space0, digit1, tag(":"),space0))(i)?;
    let (i, winners) = separated_list0(space1, digit1)(i)?;
    let (i, _) = tuple((space0, tag("|"), space0))(i)?;
    let (i, my_numbers) = separated_list0(space1, digit1)(i)?;

    let winners: HashSet<&&str> = HashSet::from_iter(winners.iter());

    let mut num = my_numbers
        .iter()
        .filter(|x| winners.contains(*x))
        .count() as u32;

    if num > 0 {
        num = u32::pow(2, num-1)
    }

    Ok((i, num))
}

pub fn process(input: &str) -> String {
    input
        .lines()
        .map(|i| {
            parse_card(i).unwrap().1
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("13", process(input));
    }
}
