#![allow(unused_variables,unused_imports,dead_code, unused_mut)]

use std::collections::{BTreeMap, VecDeque};

use nom::{IResult, Parser, character::complete::{alphanumeric1, line_ending, self, digit1, alpha1, anychar}, multi::{separated_list1, count}, sequence::{separated_pair, tuple}, branch::alt};
use nom_supreme::{tag::complete::tag, ParserExt};
use itertools::Itertools;
use rayon::{prelude::*, iter::ParallelDrainFull};
use indicatif::{ProgressIterator, ParallelProgressIterator};


fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input, "a");
    dbg!(output);
}

type Wire<'a> = &'a str;
type Signal = u16;
type Wires<'a> = BTreeMap<Wire<'a>, Signal>;

#[derive(Debug)]
enum Instruction<'a>  {
    Direct(Wire<'a>, Wire<'a>),
    DirectC(Signal, Wire<'a>),
    And(Wire<'a>, Wire<'a>, Wire<'a>),
    AndC(Wire<'a>, Signal, Wire<'a>),
    Or(Wire<'a>, Wire<'a>, Wire<'a>),
    LShift(Wire<'a>, u8, Wire<'a>),
    RShift(Wire<'a>, u8, Wire<'a>),
    Not(Wire<'a>, Wire<'a>),
}

impl<'a> Instruction<'a> {
    fn apply(&self, wires: &mut Wires<'a>) -> Option<()>
    {
        use Instruction::*;

        match self {
            Direct(a, out) => if wires.contains_key(a) {
                wires.insert(out, wires[a]);
                Some(())
            } else {
                None
            }
            DirectC(a, out) => {
                wires.insert(out, *a);
                Some(())
            }
            And(a, b, out) => 
                if wires.contains_key(a) && wires.contains_key(b) {
                    wires.insert(out, wires[a] & wires[b]);
                    Some(())
                } else {
                    None
                }
            AndC(a, b, out) => if wires.contains_key(a) {
                    wires.insert(out, wires[a] & b);
                    Some(())
                } else {
                    None
                }
            Or(a, b, out) => if wires.contains_key(a) && wires.contains_key(b) {
                wires.insert(out, wires[a] | wires[b]);
                Some(())
            } else {
                None
            }
            LShift(a, sz, out) => if wires.contains_key(a) {
                wires.insert(out, wires[a] << sz);
                Some(())
            } else {
                None
            }
            RShift(a, sz, out) => if wires.contains_key(a) {
                wires.insert(out, wires[a] >> sz);
                Some(())
            } else {
                None
            }
            Not(a, out) => if wires.contains_key(a) {
                wires.insert(out, !wires[a]);
                Some(())
            } else {
                None
            }
        }
    }
}

fn parse(i: &str) -> IResult<&str, Vec<Instruction>> {
    use Instruction::*;

    separated_list1(
        line_ending,
        alt((
            tuple((
                alpha1.terminated(tag(" -> ")),
                alpha1
            )).map(|(a, out)| Direct(a, out)),
            tuple((
                complete::u16.terminated(tag(" -> ")),
                alpha1
            )).map(|(signal, out)| DirectC(signal, out)),
            tuple((
                alpha1.terminated(tag(" AND ")),
                alpha1.terminated(tag(" -> ")),
                alpha1,
            )).map(|(a, b, out)| And(a, b, out)),
            tuple((
                complete::u16.terminated(tag(" AND ")),
                alpha1.terminated(tag(" -> ")),
                alpha1,
            )).map(|(a, b, out)| AndC(b, a, out)),
            tuple((
                alpha1.terminated(tag(" OR ")),
                alpha1.terminated(tag(" -> ")),
                alpha1,
            )).map(|(a, b, out)| Or(a, b, out)),
            tuple((
                alpha1.terminated(tag(" LSHIFT ")),
                complete::u8.terminated(tag(" -> ")),
                alpha1,
            )).map(|(a, b, out)| LShift(a, b, out)),
            tuple((
                alpha1.terminated(tag(" RSHIFT ")),
                complete::u8.terminated(tag(" -> ")),
                alpha1,
            )).map(|(a, b, out)| RShift(a, b, out)),
            tuple((
                alpha1.preceded_by(tag("NOT ")).terminated(tag(" -> ")),
                alpha1
            )).map(|(a, out)| Not(a, out)),
        ))
    ).parse(i)
}

pub fn process(input: &str, result_name: &str) -> String {
    let (i, instructions) = parse(input).unwrap();
    if i != "" {
        panic!("Remaining input {}", i);
    }
    let mut instructions: VecDeque<Instruction> = VecDeque::from_iter(instructions);
    let mut wires: BTreeMap<&str, u16> = BTreeMap::new();

    while let Some(instruction) = instructions.pop_front() {
        if instruction.apply(&mut wires).is_none() {
            instructions.push_back(instruction);
        }
    }
    wires[result_name].to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "123 -> x
x AND y -> d
x OR y -> e
1 AND y -> j
x LSHIFT 2 -> f
y RSHIFT 2 -> g
456 -> y
NOT x -> h
NOT y -> i";
        assert_eq!("72", process(input, "d"));
        assert_eq!("507", process(input, "e"));
        assert_eq!("492", process(input, "f"));
        assert_eq!("114", process(input, "g"));
        assert_eq!("65412", process(input, "h"));
        assert_eq!("65079", process(input, "i"));

    }
}
