#![allow(unused_variables,unused_imports,dead_code, unused_mut)]

use std::collections::HashSet;

use nom::{IResult, Parser, character::complete::one_of, multi::many1};

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

enum Direction {
    North,
    East,
    South,
    West
}

type Coord = (isize, isize);

fn parse(i: &str) -> IResult<&str, Vec<Direction>> {
    use Direction::*;

    many1(
        one_of("^v<>")
            .map(|ch| 
                match ch {
                    '^' => North,
                    '>' => East,
                    'v' => South,
                    '<' => West,
                    _ => unreachable!()
            })
        ).parse(i)

}

pub fn process(input: &str) -> String {
    use Direction::*;

    let (_, directions) = parse(input).unwrap();
    let mut visited: HashSet<Coord> = HashSet::new();
    let start:Coord = (0,0);
    visited.insert(start);
    directions.iter()
        .fold(start, |(x, y), dir| {
            let coord = match dir {
                North => (x, y-1),
                East => (x+1, y),
                South => (x, y+1),
                West => (x-1, y)
            };
            visited.insert(coord);
            coord
        });

    visited.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        assert_eq!("2", process(">"));
        assert_eq!("4", process("^>v<"));
        assert_eq!("2", process("^v^v^v^v^v"));
    }
}
