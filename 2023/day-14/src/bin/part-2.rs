#![allow(unused_variables,unused_imports,dead_code, unused_mut)]

use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use nom::{IResult, Parser, character::complete::line_ending, multi::{separated_list1, many1}, branch::alt};
use nom_supreme::tag::complete::tag;


fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
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

fn tilt_north(platform: &mut Platform) {
    use Tile::*;

    let width = platform[0].len();
    let height = platform.len();

    (0..width).for_each(|x| {       
        (1..height).for_each(|y| {
            if platform[y][x] == Round && platform[y-1][x] == Empty {
                let mut y_dst = y-1;
                while y_dst > 0 && platform[y_dst-1][x] == Empty {
                        y_dst -= 1;
                    }
                platform[y][x] = Empty;
                platform[y_dst][x] = Round;
            }
        })
    })
}

fn tilt_west(platform: &mut Platform) {
    use Tile::*;

    let width = platform[0].len();
    let height = platform.len();

    (0..height).for_each(|y| {
        (1..width).for_each(|x| {       
            if platform[y][x] == Round && platform[y][x-1] == Empty {
                let mut x_dst = x-1;
                while x_dst > 0 && platform[y][x_dst-1] == Empty {
                        x_dst -= 1;
                    }
                platform[y][x] = Empty;
                platform[y][x_dst] = Round;
            }
        })
    })
}

fn tilt_south(platform: &mut Platform) {
    use Tile::*;

    let width = platform[0].len();
    let height = platform.len();

    (0..width).for_each(|x| {       
        (0..height-1).rev().for_each(|y| {
            if platform[y][x] == Round && platform[y+1][x] == Empty {
                let mut y_dst = y+1;
                while y_dst < height-1 && platform[y_dst+1][x] == Empty {
                        y_dst += 1;
                    }
                platform[y][x] = Empty;
                platform[y_dst][x] = Round;
            }
        })
    })
}

fn tilt_east(platform: &mut Platform) {
    use Tile::*;

    let width = platform[0].len();
    let height = platform.len();

    (0..height).for_each(|y| {
        (0..width-1).rev().for_each(|x| {       
            if platform[y][x] == Round && platform[y][x+1] == Empty {
                let mut x_dst = x+1;
                while x_dst < width-1 && platform[y][x_dst+1] == Empty {
                        x_dst += 1;
                    }
                platform[y][x] = Empty;
                platform[y][x_dst] = Round;
            }
        })
    })
}

fn cycle_platform(platform: &mut Platform) {
    tilt_north(platform);
    tilt_west(platform);
    tilt_south(platform);
    tilt_east(platform);
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

fn hash_platform(platform: &Platform) -> u64 {
    let mut hash = DefaultHasher::new();
    platform.hash(&mut hash);
    hash.finish()
}

fn dbg_platform(platform: &Platform) {
    use Tile::*;

    for row in platform {
        for tile in row {        
            print!("{}", match tile {
                Empty => '.',
                Cube => '#',
                Round => 'O',
            });
        }
        println!("");
    }
    println!("");
}

pub fn process(i: &str) -> String {
    let (_, mut platform) = parse(i).unwrap();
    let mut seen: HashSet<u64> = HashSet::new();
    let mut cycles = 0;

    seen.insert(hash_platform(&platform));

    // find initial loop
    loop {
        cycle_platform(&mut platform);
        cycles += 1;
        if !seen.insert(hash_platform(&platform)) {
            break;
        }
    }

    let initial = cycles;

    // find stable loop
    seen.clear();
    seen.insert(hash_platform(&platform));

    loop {
        cycle_platform(&mut platform);
        cycles += 1;
        if !seen.insert(hash_platform(&platform)) {
            break;
        }
    }
    let stable = cycles - initial;

    // skip stable loop, get remaining count
    let remaining = (1_000_000_000 - initial) % stable;  

    for _ in 0..remaining {
        cycle_platform(&mut platform);
    }

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
        assert_eq!("64", process(input));
    }
}
