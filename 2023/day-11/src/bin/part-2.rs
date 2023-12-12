#![allow(unused_variables,unused_imports,dead_code)]

use std::collections::BTreeSet;
use itertools::Itertools;

use glam::IVec2;

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input, 1_000_000);
    dbg!(output);
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Galaxy {
    x: usize,
    y: usize,
}

type Universe = BTreeSet<Galaxy>;

fn parse(i: &str) -> (IVec2, Universe) {   
    let universe = i.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, ch)| {
                    match ch {
                        '#' => Some(Galaxy{x, y}),
                        '.' => None,
                        _ => unreachable!("invalid character")
                    }    
                })
        })
        .collect::<Universe>();
    let height = i.lines().count() as i32;
    let width = i.lines().next().unwrap().chars().count() as i32;

    (IVec2::new(width, height), universe)
    
}

fn get_row_and_column_mapping(dimension: &IVec2, universe: &Universe, stretch: usize) -> (Vec<usize>, Vec<usize>) {
    let col_map = (0..dimension.x as usize)
        .scan(
            0 as usize,
            |acc, x| {
                if (0..dimension.y as usize).any(|y| universe.contains(&Galaxy{x, y})) {
                    *acc += 1;
                } else {
                    *acc += stretch;
                }
                Some(*acc)
            }
        )
        .collect::<Vec<_>>();
    let row_map = (0..dimension.y as usize)
        .scan(
            0,
            |acc, y| {
                if (0..dimension.x as usize).any(|x| universe.contains(&Galaxy{x, y})) {
                    *acc += 1;
                } else {
                    *acc += stretch;
                }
                Some(*acc)
            }
        )
        .collect::<Vec<_>>();
    (row_map, col_map)
}

pub fn process(i: &str, stretch: usize) -> String {
    let (dimension, universe) = parse(i);
    let (row_map, column_map) = get_row_and_column_mapping(&dimension, &universe, stretch);

    universe
        .iter()
        .combinations(2)
        .map(|g| {
            let dx = column_map[g[0].x as usize].abs_diff(column_map[g[1].x as usize]);
            let dy = row_map[g[0].y as usize].abs_diff(row_map[g[1].y as usize]);
            dx + dy
        })
        .sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "\
#..
...
..#";
        assert_eq!("2000002", process(input, 1_000_000));
    }
    #[test]
    fn test_process_2() {
        let input = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!("1030", process(input, 10));
        assert_eq!("8410", process(input, 100));
    }
}
