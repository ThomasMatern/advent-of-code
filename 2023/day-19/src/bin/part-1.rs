#![allow(unused_variables,unused_imports,dead_code)]

use std::collections::BTreeMap;

use nom::{IResult, Parser, character::complete::{self, line_ending, one_of, alpha1}, multi::separated_list1, sequence::{separated_pair, delimited, tuple}, branch::alt};
use nom_supreme::{tag::complete::tag, ParserExt};

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

enum Condition<'a> {
    Check(&'a str, char, i32, &'a str),
    Unconditional(&'a str),
}

struct Workflow<'a> {
    rules: Vec<Condition<'a>>,
}

#[derive(Debug, Copy, Clone)]
struct Part {
    x: i32,
    m: i32, 
    a: i32,
    s: i32,
}

fn parse(i: &str) -> IResult<&str, (BTreeMap<&str, Workflow>, Vec<Part>)> {
    separated_pair(
        separated_list1(
            line_ending,
            tuple((
                alpha1,
                delimited(
                    tag("{"), 
                    separated_list1(
                        tag(","),
                        alt((
                            tuple((
                                alpha1, 
                                one_of("<>"),
                                complete::i32.terminated(tag(":")),
                                alpha1
                            )).map(|(var, test, value, target)| Condition::Check(var, test, value, target)),
                            alpha1.map(|target| Condition::Unconditional(target)),
                        ))
                    ),
                    tag("}"))
                )).map(|(name, rules)| (name, Workflow{rules}))
        ).map(|workflows| BTreeMap::from_iter(workflows)),
        tuple((line_ending, line_ending)),
        separated_list1(
            line_ending,
            delimited(
                tag("{"),
                tuple((
                    complete::i32.preceded_by(tag("x=")).terminated(tag(",")),
                    complete::i32.preceded_by(tag("m=")).terminated(tag(",")),
                    complete::i32.preceded_by(tag("a=")).terminated(tag(",")),
                    complete::i32.preceded_by(tag("s=")),
                )).map(|(x, m, a, s)| Part{x, m, a, s}),

                
                tag("}")
            )
        )
        ).parse(i)
}

pub fn process(input: &str) -> String {
    let (_, (workflows, parts)) = parse(input).unwrap();

    let mut result = 0;

    for part in parts {
        let mut dest = "in";
        
        loop {
            let workflow = &workflows[dest];
            for rule in &workflow.rules {
                match *rule {
                    Condition::Check(var, test, value, target) => {
                        if match (var, test) {
                            ("x", '<') => part.x < value,
                            ("x", '>') => part.x > value,
                            ("m", '<') => part.m < value,
                            ("m", '>') => part.m > value,
                            ("a", '<') => part.a < value,
                            ("a", '>') => part.a > value,
                            ("s", '<') => part.s < value,
                            ("s", '>') => part.s > value,
                            x => panic!("Unhandled {:?}", x)
                        } {
                            dest = target;
                            break
                        }
                    },
                    Condition::Unconditional(target) => {
                        dest = target;
                        break
                    }
                }
            }
            if dest == "A" {
                result += part.x + part.m + part.a + part.s;
                break
            }
            if dest == "R" {
                break
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!("19114", process(input));
    }
}
