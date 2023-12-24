#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

use std::collections::BTreeMap;
use nom::{
    IResult,
    Parser,
    character::complete::{ alphanumeric1, line_ending },
    multi::{ separated_list1, count },
    sequence::separated_pair,
};
use nom_supreme::{ tag::complete::tag, ParserExt };

use num_integer::lcm;

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

type Map<'a> = BTreeMap<&'a str, &'a str>;

fn parse(i: &str) -> IResult<&str, (&str, Map, Map)> {
    let (i, instructions) = alphanumeric1.terminated(count(line_ending, 2)).parse(i)?;

    let (i, map) = separated_list1(
        line_ending,
        separated_pair(
            alphanumeric1,
            tag(" = "),
            separated_pair(
                alphanumeric1.preceded_by(tag("(")),
                tag(", "),
                alphanumeric1.terminated(tag(")"))
            )
        )
    ).parse(i)?;

    let map_l: BTreeMap<&str, &str> = BTreeMap::from_iter(map.iter().map(|(k, (l, _))| (*k, *l)));
    let map_r: BTreeMap<&str, &str> = BTreeMap::from_iter(map.iter().map(|(k, (_, r))| (*k, *r)));
    Ok((i, (instructions, map_l, map_r)))
}

fn run_to_z<'a>(
    pos: &'a str,
    instructions: &mut dyn Iterator<Item = char>,
    map_l: &BTreeMap<&str, &'a str>,
    map_r: &BTreeMap<&str, &'a str>
) -> (&'a str, u128) {
    let mut pos = pos;
    let Some(step_count) = instructions.enumerate().find_map(|(step, dir)| {
        let map = match dir {
            'L' => &map_l,
            'R' => &map_r,
            _ => panic!("bad direction"),
        };
        pos = map.get(pos).unwrap();
        if pos.ends_with('Z') {
            Some(step + 1)
        } else {
            None
        }
    }) else {
        panic!("should not happen");
    };
    (pos, step_count as u128)
}

pub fn lcmm(values: Vec<u128>) -> u128 {
    let mut result = values[0];
    for value in values[1..].iter() {
        result = lcm(result, *value);
    }
    result
}

pub fn process(input: &str) -> String {
    let (_, (instructions, map_l, map_r)) = parse(input).unwrap();

    let positions = map_l
        .keys()
        .filter_map(|a| a.ends_with('A').then_some(*a))
        .collect::<Vec<_>>();

    let factors = positions
        .iter()
        .map(|pos| {
            let mut iter = instructions.chars().cycle();
            let (pos, first) = run_to_z(pos, &mut iter, &map_l, &map_r);
            let (_, second) = run_to_z(pos, &mut iter, &map_l, &map_r);
            if first != second {
                panic!("input data unsuitable for this solution");
            }
            first
        })
        .collect::<Vec<_>>();

    let cycles = lcmm(factors);
    cycles.to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process_1() {
        let input =
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!("6", process(input));
    }
}
