#![allow(unused_variables,unused_imports,dead_code, unused_mut)]

use nom::{IResult, Parser, character::complete::{alphanumeric1, one_of, digit0}, multi::separated_list1, sequence::tuple};
use nom_supreme::tag::complete::tag;


fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, Copy, Clone)]
struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
}

#[derive(Debug)]
struct Step<'a> {
    lens: Lens<'a>,
    operation: char,
}

fn parse(i: &str) -> IResult<&str, Vec<Step>> {
    separated_list1(
        tag(","), 
        tuple((
            alphanumeric1,
            one_of("=-"),
            digit0
            ))
            .map(|(label, operation, focal_length)| {
                let lens = Lens {label, focal_length: focal_length.parse().unwrap_or(0) };
                Step {lens, operation}
            })
        )
        .parse(i)
}

fn hash_one(mut acc: usize, value: usize) -> usize {
    acc += value;
    acc *= 17;
    acc %= 256;
    acc
}

fn hash(value: &str) -> usize {
    value.chars()
        .fold(0, |acc, item| {
            hash_one(acc, item as usize)
        })
}

type ElfHashMap<'a> = Vec<Vec<Lens<'a>>>;

fn lens_position(box_content: &Vec<Lens>, lens: &Lens) -> Option<usize> {
    box_content
        .iter()
        .enumerate()
        .find(|(_, box_lens)| box_lens.label == lens.label)
        .map(|(idx, _)| idx)
}

pub fn process(i: &str) -> String {
    let (_, seq) = parse(i).unwrap();

    let mut hashmap: ElfHashMap = vec![vec![]; 256];

    seq.iter()
        .for_each(|step| {
            let box_num = hash(step.lens.label);
            let box_content = &mut hashmap[box_num];
            match (step.operation, lens_position(box_content, &step.lens)) {
                ('=', None) => box_content.push(step.lens),
                ('=', Some(pos)) => {
                    let _ = std::mem::replace(&mut box_content[pos], step.lens); 
                    ()
                },
                ('-', None) => (),
                ('-', Some(pos)) => {
                    let _ = box_content.remove(pos);
                    ()
                },
                _ => panic!("Invalid operation"),
            };
        });

    hashmap.iter()
        .enumerate()
        .flat_map(|(box_idx, box_content)| 
            box_content
                .iter()
                .enumerate()
                .map(move |(lens_idx, lens)| 
                    (box_idx+1) * (lens_idx+1) * lens.focal_length
                )
            )
        .sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!("145", process(input));
    }
}
