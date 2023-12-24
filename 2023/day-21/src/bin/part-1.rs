#![allow(unused_variables,unused_imports,dead_code, unused_mut)]

use std::collections::BTreeMap;

use nom::{IResult, Parser, character::complete::{self, line_ending}, multi::{separated_list1, many1}, branch::alt};

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input, 64);
    dbg!(output);
}

#[derive(Debug, PartialEq)]
enum Tile {
    Garden,
    Rocks,
    Start,
}

type Coord = (usize, usize);
type Garden = Vec<Vec<Tile>>;
type Visited = BTreeMap<Coord, usize>;

impl Tile {
    fn visit(garden: &Garden, visited: &mut Visited, coord: Coord, step: usize) -> bool {
        if !visited.contains_key(&coord) {
            return false;
        }
        if *visited.get(&coord).unwrap() != step-1 {
            return false;
        }
        let x = coord.0;
        let y = coord.1;
        let mut modified = false;

        for (dx, dy) in [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)] {
            if (x==0 && dx == -1) || (y==0 && dy == -1) {
                continue
            }
            let xt = (x as isize + dx) as usize;
            let yt = (y as isize + dy) as usize;

            if xt >= garden[0].len() || yt >= garden.len() {
                continue
            }

            if garden[yt][xt] == Tile::Rocks {
                continue
            }
            
            if visited.contains_key(&(xt, yt)) {
                continue;
            }

            visited.insert((xt, yt), step);
            modified = true;
        }
        modified
    }
}


fn parse(i: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list1(
        line_ending,
        many1(
            alt((
                complete::char('.').map(|_| Tile::Garden),
                complete::char('#').map(|_| Tile::Rocks),
                complete::char('S').map(|_| Tile::Start),
            ))
        )
        ).parse(i)
}

pub fn process(input: &str, num_steps: usize) -> String {
    let (_, mut garden) = parse(input).unwrap();
    let mut start: Coord = (0, 0);

    let width = garden[0].len();
    let height = garden.len();
    
    garden.iter()
        .enumerate()
        .for_each(|(y, row)| 
            row.iter()
                .enumerate()
                .for_each(|(x, t)|
                    if *t == Tile::Start {
                        start = (x, y);
                    }
                )
            );
    garden[start.1][start.0] = Tile::Garden;

    let mut visited: Visited = BTreeMap::new();
    visited.insert(start, 0);


    for step in 1.. {
        let mut modified = false;
        for x in 0..width {
            for y in 0..height {
                modified |= Tile::visit(&garden, &mut visited, (x, y), step);
            }
        }
        if !modified {
            break
        }
    }
    visited.iter()
        .filter(|(&(x, y), &cnt)| {
            if cnt <= num_steps && cnt % 2 == 0 {
                println!("{}  {}  {}", x, y, cnt);
                true
            } else {
                false
            }
        })
        .count().to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!("16", process(input, 6));
    }
}
