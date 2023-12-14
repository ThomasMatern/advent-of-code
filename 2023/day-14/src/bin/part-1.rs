#![allow(unused_variables,unused_imports,dead_code, unused_mut)]

use nom::{IResult, Parser, character::complete::line_ending, multi::{separated_list1, many1}, branch::alt};
use nom_supreme::tag::complete::tag;
use rayon::prelude::*;
use indicatif::{ProgressIterator, ParallelProgressIterator};


fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, PartialEq)]
enum Tile {
    Empty,
    Cube,
    Round
}

type Platform = Vec<Vec<Tile>>;

fn parse(i: &str) -> IResult<&str, Platform> {
    use Tile::*;

    separated_list1(
        line_ending,
        many1(
            alt((
                tag(".").map(|_| Empty),
                tag("#").map(|_| Cube),
                tag("O").map(|_| Round)
                ))
            )
        ).parse(i)
}

fn tilt_up(platform: &mut Platform) {
    use Tile::*;

    let width = platform[0].len();
    let height = platform.len();

    loop {
        let mut modified = false;
        for x in 0..width {
            for y in 1..height {
                if platform[y][x] == Round && platform[y-1][x] == Empty {
                    let mut y_top = y-1;
                    while y_top > 0 && platform[y_top-1][x] == Empty {
                         y_top -= 1;
                     }
                    platform[y][x] = Empty;
                    platform[y_top][x] = Round;
                    modified = true
                }
            }
        }
        if !modified {
            return;
        }
    }
}

fn score_platform(platform: &Platform) -> usize {
    use Tile::*;
    let height = platform.len();

    platform.iter()
        .enumerate()
        .flat_map(|(y, row)| 
            row.iter()
                .filter(|&tile| tile == &Round)
                .map(move |_| height - y)
        )
        .sum()
}

pub fn process(i: &str) -> String {
    let (_, mut platform) = parse(i).unwrap();
    tilt_up(&mut platform);
    score_platform(&platform).to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!("136", process(input));
    }
}
