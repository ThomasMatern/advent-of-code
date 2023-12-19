#![allow(unused_variables,unused_imports,dead_code, unused_mut)]

use nom::{IResult, Parser, character::complete::{alphanumeric1, line_ending, self, digit1}, multi::{separated_list1, count}, sequence::{separated_pair, tuple}, branch::alt};
use nom_supreme::{tag::complete::tag, ParserExt};
use itertools::Itertools;
use rayon::{prelude::*, iter::ParallelDrainFull};
use indicatif::{ProgressIterator, ParallelProgressIterator};


fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

type Coord = (usize, usize);
type Map = Vec<Vec<usize>>;

enum Instruction {
    On(Coord, Coord),
    Off(Coord, Coord),
    Toggle(Coord, Coord),
}

impl Instruction {
    fn apply(&self, map: &mut Map) {
        use Instruction::*;
        match self {
            On(tl, br) => (tl.0..=br.0).cartesian_product(tl.1..=br.1)
                .for_each(|(x, y)| map[y][x] += 1),
            Off(tl, br) => (tl.0..=br.0).cartesian_product(tl.1..=br.1)
            .for_each(|(x, y)| if map[y][x] > 0 {
                map[y][x] -= 1}
            ),
            Toggle(tl, br) => (tl.0..=br.0).cartesian_product(tl.1..=br.1)
                .for_each(|(x, y)| map[y][x] += 2),
        }
    }
}

fn parse(i: &str) -> IResult<&str, Vec<Instruction>> {
    use Instruction::*;

    separated_list1(
        line_ending, 
        tuple((
            alt((
                tag("turn on"),
                tag("turn off"),
                tag("toggle")
            )),
            complete::u32.preceded_by(tag(" ")).terminated(tag(",")).map(|v| v as usize),
            complete::u32.terminated(tag(" through ")).map(|v| v as usize),
            complete::u32.terminated(tag(",")).map(|v| v as usize),
            complete::u32.map(|v| v as usize)
        )).map(|(op, x1, y1, x2, y2)|
            match op {
                "turn on" => On((x1, y1), (x2, y2)),
                "turn off" => Off((x1, y1), (x2, y2)),
                "toggle" => Toggle((x1, y1), (x2, y2)),
                _ => panic!("bad instruction")
            }
        )
    ).parse(i)
}

pub fn process(input: &str) -> String {
    let (_, instructions) = parse(input).unwrap();
    let mut map: Map = vec![vec![0; 1000]; 1000];

    for instruction in instructions {
        instruction.apply(&mut map);
    }
    (0usize ..1000).cartesian_product(0usize ..1000).map(|(x, y)| map[y][x]).sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "toggle 0,0 through 99,99";
        assert_eq!("20000", process(input));
    }
}
