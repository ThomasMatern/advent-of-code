#![allow(unused_variables,unused_imports,dead_code, unused_mut)]

use std::{collections::BTreeMap, iter};

use nom::{IResult, Parser, character::complete::{alphanumeric1, line_ending}, multi::{separated_list1, count}, sequence::separated_pair};
use nom_supreme::{tag::complete::tag, ParserExt};


fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}


fn parse(i: &str) -> IResult<&str, (&str, BTreeMap<&str, (&str, &str)>)> {
    let (i, instructions) = alphanumeric1
        .terminated(count(line_ending, 2))
        .parse(i)?;

    let (i, map) = separated_list1(line_ending, 
        separated_pair(
            alphanumeric1,
            tag(" = "), 
            separated_pair(
                alphanumeric1.preceded_by(tag("(")),
                tag(", "),
                alphanumeric1.terminated(tag(")"))
            ))
        )
        .parse(i)?;
    let map: BTreeMap<&str, (&str, &str)> = BTreeMap::from_iter(map);
    
    Ok((i, (instructions, map)))
}



pub fn process(input: &str) -> String {
    let (_, (instructions, map)) = parse(input).unwrap();

    let mut pos = "AAA";
    let mut instructions = iter::repeat(instructions).flat_map(|i| i.chars());
    for step in 1.. {
        let node = map.get(pos).expect("bad position");
        let direction = instructions.next().expect("missing direction");
        pos = match direction {
            'L' => node.0,
            'R' => node.1,
            _ => panic!("bad direction"),
        };
        if pos == "ZZZ" {
            return step.to_string();
        }
    }
    panic!("Should never reach here")
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("2", process(input));
    }

    #[test]
    fn test_process_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("6", process(input));
    }
}
