#![allow(unused_variables,unused_imports,dead_code)]

use std::collections::BTreeSet;
use itertools::Itertools;

use glam::IVec2;

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Galaxy {
    x: i32,
    y: i32,
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
                        '#' => Some(Galaxy{x:x as i32, y:y as i32}),
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

fn get_row_and_column_mapping(dimension: &IVec2, universe: &Universe) -> (Vec<i32>, Vec<i32>) {
    let col_map = (0..dimension.x)
        .scan(
            0,
            |acc, x| {
                if (0..dimension.y).any(|y| universe.contains(&Galaxy{x, y})) {
                    *acc += 1;
                } else {
                    *acc += 2;
                }
                Some(*acc)
            }
        )
        .collect::<Vec<_>>();
    let row_map = (0..dimension.y)
        .scan(
            0,
            |acc, y| {
                if (0..dimension.x).any(|x| universe.contains(&Galaxy{x, y})) {
                    *acc += 1;
                } else {
                    *acc += 2;
                }
                Some(*acc)
            }
        )
        .collect::<Vec<_>>();
    (row_map, col_map)
}

pub fn process(i: &str) -> String {
    let (dimension, universe) = parse(i);
    let (row_map, column_map) = get_row_and_column_mapping(&dimension, &universe);

    universe
        .iter()
        .combinations(2)
        .map(|g| {
            let dx = column_map[g[0].x as usize].abs_diff(column_map[g[1].x as usize]);
            let dy = row_map[g[0].y as usize].abs_diff(row_map[g[1].y as usize]);
            // dbg!(g, dx, dy);

            dx + dy
        })
        .sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "\
#.
..
.#";
        assert_eq!("4", process(input));
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
        assert_eq!("374", process(input));
    }
}
