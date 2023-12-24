#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

use std::collections::{ HashSet, BinaryHeap };
use nom::{
    IResult,
    Parser,
    character::complete::{ self, line_ending },
    multi::{ separated_list1, many1 },
};

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

fn parse(i: &str) -> IResult<&str, Vec<Vec<isize>>> {
    separated_list1(
        line_ending,
        many1(complete::one_of("0123456789").map(|d| d.to_digit(10).unwrap() as isize))
    ).parse(i)
}

type Coord = (isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North(u8),
    East(u8),
    South(u8),
    West(u8),
    Start,
}

impl Direction {
    fn left(&self) -> Self {
        use Direction::*;
        match self {
            North(_) => West(0),
            East(_) => North(0),
            South(_) => East(0),
            West(_) => South(0),
            Start => East(0),
        }
    }

    fn right(&self) -> Self {
        use Direction::*;
        match self {
            North(_) => East(0),
            East(_) => South(0),
            South(_) => West(0),
            West(_) => North(0),
            Start => South(0),
        }
    }

    fn num_steps(&self) -> u8 {
        use Direction::*;
        match self {
            North(s) => *s,
            East(s) => *s,
            South(s) => *s,
            West(s) => *s,
            Start => 99,
        }
    }

    fn walk(&self, coord: &Coord) -> (Coord, Self) {
        use Direction::*;
        match self {
            North(s) => ((coord.0, coord.1 - 1), North(s + 1)),
            East(s) => ((coord.0 + 1, coord.1), East(s + 1)),
            South(s) => ((coord.0, coord.1 + 1), South(s + 1)),
            West(s) => ((coord.0 - 1, coord.1), West(s + 1)),
            Start => panic!("Cannot walk in Start direction"),
        }
    }
}

type QueueEntry = (isize, Coord, Direction); // cost(negative), position, direction
type HashEntry = (Coord, Direction); // position, direction

fn in_map(coord: &Coord, map: &Vec<Vec<isize>>) -> bool {
    coord.0 >= 0 &&
        coord.1 >= 0 &&
        coord.0 < (map[0].len() as isize) &&
        coord.1 < (map.len() as isize)
}

pub fn process(input: &str) -> String {
    use Direction::*;

    let (_, map) = parse(input).unwrap();

    let mut queue: BinaryHeap<QueueEntry> = BinaryHeap::new();
    let mut seen: HashSet<HashEntry> = HashSet::new();

    queue.push((0, (0, 0), Start));
    while let Some((cost, coord, dir)) = queue.pop() {
        if !in_map(&coord, &map) {
            continue;
        }

        if coord == ((map[0].len() - 1) as isize, (map.len() - 1) as isize) {
            return (-cost).to_string();
        }

        if seen.contains(&(coord, dir)) {
            continue;
        }
        seen.insert((coord, dir));

        if dir.num_steps() < 3 {
            let (ncoord, ndir) = dir.walk(&coord);
            if in_map(&ncoord, &map) {
                queue.push((cost - map[ncoord.1 as usize][ncoord.0 as usize], ncoord, ndir));
            }
        }

        for ndir in [dir.left(), dir.right()] {
            let (ncoord, ndir) = ndir.walk(&coord);
            if in_map(&ncoord, &map) {
                queue.push((cost - map[ncoord.1 as usize][ncoord.0 as usize], ncoord, ndir));
            }
        }
    }
    panic!("Queue empty without reaching target")
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input =
            "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!("102", process(input));
    }
}
