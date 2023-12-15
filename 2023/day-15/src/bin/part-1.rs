#![allow(unused_variables,unused_imports,dead_code)]

use nom::{IResult, Parser, character::complete::alphanumeric1 , multi::{separated_list1, many1}, branch::alt};
use nom_supreme::tag::complete::tag;


fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

fn parse(i: &str) -> IResult<&str, Vec<String>> {
    separated_list1(
        tag(","), 
        many1(
            alt((alphanumeric1,
                tag("="),
                tag("-")))
            ).map(|x| x.join(""))
        )
        .parse(i)
}

fn hash(mut acc: usize, value: usize) -> usize {
    acc += value;
    acc *= 17;
    acc %= 256;
    acc
}

pub fn process(i: &str) -> String {
    let (_, seq) = parse(i).unwrap();

    seq.iter()
        .map(|step| {
            step.chars()
                .fold(0,
                    |acc, item| {
                        hash(acc, item as usize)
                    })
        })
        .sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("1320", process(input));
    }
}
