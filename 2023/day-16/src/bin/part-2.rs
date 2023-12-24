#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

use std::collections::{ HashSet, BTreeSet };

use nom::{
    IResult,
    Parser,
    character::complete::{ alphanumeric1, line_ending, one_of },
    multi::{ separated_list1, count, many1 },
    branch::alt,
    sequence::separated_pair,
};
use nom_supreme::{ tag::complete::tag, ParserExt };
use itertools::Itertools;
use rayon::{ prelude::*, iter::Empty };
use indicatif::{ ProgressIterator, ParallelProgressIterator };

#[derive(Debug)]
enum Tile {
    Empty,
    MirrorFwd,
    MirrorBack,
    SplitterH,
    SplitterV,
}

type Coord = (isize, isize);

#[derive(Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct Beam<'a> {
    coord: Coord,
    direction: &'a Direction,
}

type Map = Vec<Vec<Tile>>;

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

fn parse(i: &str) -> IResult<&str, Map> {
    use Tile::*;

    separated_list1(
        line_ending,
        many1(
            one_of(r"./\-|").map(|ch| {
                match ch {
                    '.' => Empty,
                    '-' => SplitterH,
                    '|' => SplitterV,
                    '/' => MirrorFwd,
                    '\\' => MirrorBack,
                    _ => unreachable!(""),
                }
            })
        )
    ).parse(i)
}

fn process_from(map: &Map, from: Beam) -> usize {
    use Direction::*;
    use Tile::*;

    let mut energized = vec![vec![false; map[0].len()];map.len()];
    let mut queue: BTreeSet<Beam> = BTreeSet::new();
    let mut seen: HashSet<Beam> = HashSet::new();

    queue.insert(from);

    while let Some(beam) = queue.pop_first() {
        if seen.contains(&beam) {
            continue;
        }

        let (x, y) = beam.coord;
        if x < 0 || y < 0 || x >= (map[0].len() as isize) || y >= (map.len() as isize) {
            continue;
        }
        let direction = &beam.direction;

        energized[y as usize][x as usize] = true;

        match (beam.direction, &map[y as usize][x as usize]) {
            (North, Empty | SplitterV) => {
                queue.insert(Beam { coord: (x, y - 1), direction });
            }
            (North, MirrorFwd) => {
                queue.insert(Beam { coord: (x + 1, y), direction: &East });
            }
            (North, MirrorBack) => {
                queue.insert(Beam { coord: (x - 1, y), direction: &West });
            }
            (North, SplitterH) => {
                queue.insert(Beam { coord: (x - 1, y), direction: &West });
                queue.insert(Beam { coord: (x + 1, y), direction: &East });
            }

            (East, Empty | SplitterH) => {
                queue.insert(Beam { coord: (x + 1, y), direction });
            }
            (East, MirrorFwd) => {
                queue.insert(Beam { coord: (x, y - 1), direction: &North });
            }
            (East, MirrorBack) => {
                queue.insert(Beam { coord: (x, y + 1), direction: &South });
            }
            (East, SplitterV) => {
                queue.insert(Beam { coord: (x, y - 1), direction: &North });
                queue.insert(Beam { coord: (x, y + 1), direction: &South });
            }

            (South, Empty | SplitterV) => {
                queue.insert(Beam { coord: (x, y + 1), direction });
            }
            (South, MirrorFwd) => {
                queue.insert(Beam { coord: (x - 1, y), direction: &West });
            }
            (South, MirrorBack) => {
                queue.insert(Beam { coord: (x + 1, y), direction: &East });
            }
            (South, SplitterH) => {
                queue.insert(Beam { coord: (x - 1, y), direction: &West });
                queue.insert(Beam { coord: (x + 1, y), direction: &East });
            }

            (West, Empty | SplitterH) => {
                queue.insert(Beam { coord: (x - 1, y), direction });
            }
            (West, MirrorFwd) => {
                queue.insert(Beam { coord: (x, y + 1), direction: &South });
            }
            (West, MirrorBack) => {
                queue.insert(Beam { coord: (x, y - 1), direction: &North });
            }
            (West, SplitterV) => {
                queue.insert(Beam { coord: (x, y - 1), direction: &North });
                queue.insert(Beam { coord: (x, y + 1), direction: &South });
            }
        }
        seen.insert(beam);
    }

    // energized.iter().for_each(|row| {
    //     row.iter().for_each(|&value| {
    //         if value {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     });
    //     println!("");
    // });

    energized
        .iter()
        .map(|row| {
            row.iter()
                .filter(|&value| *value)
                .count()
        })
        .sum::<usize>()
}

pub fn process(i: &str) -> String {
    use Direction::*;
    use Tile::*;

    let (_, map) = parse(i).unwrap();

    let width = map[0].len() as isize;
    let height = map.len() as isize;

    let mut best = 0;
    for x in 0..width {
        best = best.max(process_from(&map, Beam { coord: (x, 0), direction: &South }));
        best = best.max(process_from(&map, Beam { coord: (x, height - 1), direction: &North }));
    }
    for y in 0..height {
        best = best.max(process_from(&map, Beam { coord: (0, y), direction: &East }));
        best = best.max(process_from(&map, Beam { coord: (width - 1, y), direction: &West }));
    }

    best.to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input =
            r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!("51", process(input));
    }
}
