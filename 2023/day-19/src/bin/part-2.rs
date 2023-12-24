#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

use std::collections::{ BTreeMap, VecDeque };

use nom::{
    IResult,
    Parser,
    character::complete::{ self, line_ending, one_of, alpha1 },
    multi::separated_list1,
    sequence::{ separated_pair, delimited, tuple },
    branch::alt,
};
use nom_supreme::{ tag::complete::tag, ParserExt };

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

enum Condition<'a> {
    Check(&'a str, char, i64, &'a str),
    Unconditional(&'a str),
}

struct Workflow<'a> {
    rules: Vec<Condition<'a>>,
}

fn parse(i: &str) -> IResult<&str, (BTreeMap<&str, Workflow>, Vec<()>)> {
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
                                complete::i64.terminated(tag(":")),
                                alpha1,
                            )).map(|(var, test, value, target)|
                                Condition::Check(var, test, value, target)
                            ),
                            alpha1.map(Condition::Unconditional),
                        ))
                    ),
                    tag("}")
                ),
            )).map(|(name, rules)| (name, Workflow { rules }))
        ).map(BTreeMap::from_iter),
        tuple((line_ending, line_ending)),
        separated_list1(
            line_ending,
            delimited(
                tag("{"),
                tuple((
                    complete::i64.preceded_by(tag("x=")).terminated(tag(",")),
                    complete::i64.preceded_by(tag("m=")).terminated(tag(",")),
                    complete::i64.preceded_by(tag("a=")).terminated(tag(",")),
                    complete::i64.preceded_by(tag("s=")),
                )).map(|_| ()),
                tag("}")
            )
        )
    ).parse(i)
}

#[derive(Debug)]
struct PartRange<'a> {
    workflow: &'a str,
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

pub fn process(input: &str) -> String {
    let (_, (workflows, _)) = parse(input).unwrap();

    let mut queue: VecDeque<PartRange> = VecDeque::new();
    let mut accepted: Vec<PartRange> = Vec::new();

    queue.push_back(PartRange {
        workflow: "in",
        x: (1, 4001),
        m: (1, 4001),
        a: (1, 4001),
        s: (1, 4001),
    });

    while let Some(part) = queue.pop_front() {
        let (mut x, mut m, mut a, mut s) = (part.x, part.m, part.a, part.s);

        if x.0 >= x.1 || m.0 >= m.1 || a.0 >= a.1 || s.0 >= s.1 {
            continue;
        }

        if part.workflow == "R" {
            continue;
        }

        if part.workflow == "A" {
            accepted.push(part);
            continue;
        }

        let workflow = &workflows[part.workflow];

        for rule in &workflow.rules {
            match *rule {
                Condition::Check(var, test, value, target) => {
                    match (var, test) {
                        ("x", '<') => {
                            queue.push_back(PartRange {
                                workflow: target,
                                x: (x.0, value),
                                m,
                                a,
                                s,
                            });
                            x = (value, x.1);
                        }
                        ("x", '>') => {
                            queue.push_back(PartRange {
                                workflow: target,
                                x: (value + 1, x.1),
                                m,
                                a,
                                s,
                            });
                            x = (x.0, value + 1);
                        }
                        ("m", '<') => {
                            queue.push_back(PartRange {
                                workflow: target,
                                x,
                                m: (m.0, value),
                                a,
                                s,
                            });
                            m = (value, m.1);
                        }
                        ("m", '>') => {
                            queue.push_back(PartRange {
                                workflow: target,
                                x,
                                m: (value + 1, m.1),
                                a,
                                s,
                            });
                            m = (m.0, value + 1);
                        }
                        ("a", '<') => {
                            queue.push_back(PartRange {
                                workflow: target,
                                x,
                                m,
                                a: (a.0, value),
                                s,
                            });
                            a = (value, a.1);
                        }
                        ("a", '>') => {
                            queue.push_back(PartRange {
                                workflow: target,
                                x,
                                m,
                                a: (value + 1, a.1),
                                s,
                            });
                            a = (a.0, value + 1);
                        }
                        ("s", '<') => {
                            queue.push_back(PartRange {
                                workflow: target,
                                x,
                                m,
                                a,
                                s: (s.0, value),
                            });
                            s = (value, s.1);
                        }
                        ("s", '>') => {
                            queue.push_back(PartRange {
                                workflow: target,
                                x,
                                m,
                                a,
                                s: (value + 1, s.1),
                            });
                            s = (s.0, value + 1);
                        }
                        _ => panic!(),
                    }
                }
                Condition::Unconditional(target) => {
                    queue.push_back(PartRange { workflow: target, x, m, a, s });
                }
            }
        }
    }
    accepted
        .iter()
        .map(
            |part|
                (part.x.1 - part.x.0) *
                (part.m.1 - part.m.0) *
                (part.a.1 - part.a.0) *
                (part.s.1 - part.s.0)
        )
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input =
            "px{a<2006:qkq,m>2090:A,rfg}
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
        assert_eq!("167409079868000", process(input));
    }
}
