#![allow(unused_variables,unused_imports,dead_code, unused_mut)]

use nom::{IResult, Parser, character::complete::{self, alphanumeric1, line_ending, one_of}, multi::separated_list1, sequence::{tuple, delimited}};
use nom_supreme::{tag::complete::tag, ParserExt};

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn direction(&self) -> (isize, isize) {
        use Direction::*;

        match self {
            Up => (0, -1),
            Right => (1, 0),
            Down => (0, 1),
            Left => (-1, 0),
        }
    }
}
struct Instruction {
    direction: Direction,
    distance: isize,
}

fn parse(i: &str) -> IResult<&str, Vec<Instruction>> {
    use Direction::*;

    separated_list1(
        line_ending,
        tuple((
            one_of("URDL").terminated(tag(" ")).map(|dir| 
            match dir {
                'U' => Up,
                'R' => Right,
                'D' => Down,
                'L' => Left,
                _ => panic!("Bad direction"),
            }),
            complete::u32.terminated(tag(" ")).map(|d| d as isize),
            delimited(tag("(#"), alphanumeric1, tag(")"))
        )).map(|(direction, distance, _)| Instruction { direction, distance} )
    ).parse(i)
}

type Coord = (isize, isize);

pub fn process(input: &str) -> String {
    let (_, instructions) = parse(input).unwrap();
    let mut polygon: Vec<Coord> = vec![];
    let mut pos:Coord = (0, 0);
    let mut boundary_length = 0;

    for instruction in instructions {
        let direction = instruction.direction.direction();
        polygon.push(pos);
        pos = (pos.0+direction.0*instruction.distance, pos.1+direction.1*instruction.distance);
        boundary_length += instruction.distance;
    }

    // shoelace
    let area = (0..polygon.len()).map(|idx| {
        let prev_idx = (idx + polygon.len() - 1) % polygon.len();
        let next_idx = (idx + 1) % polygon.len();

        polygon[idx].0 * (polygon[prev_idx].1 - polygon[next_idx].1)

    }).sum::<isize>().abs() / 2;

    // Pick's
    let interior = area - boundary_length / 2 + 1;

    (interior + boundary_length).to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!("62", process(input));
    }
}
