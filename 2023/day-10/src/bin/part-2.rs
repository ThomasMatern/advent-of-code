#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

use std::collections::{ BTreeMap, BTreeSet };

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

type Coord = (isize, isize);
type Map = BTreeMap<Coord, Vec<Coord>>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum TileType {
    Horizontal,
    Vertical,
    CornerN,
    CornerS,
}

type TileTypes = BTreeMap<Coord, TileType>;

fn add_tile(m: &mut Map, coord: Coord, offset_1: Coord, offset_2: Coord) {
    m.insert(
        coord,
        vec![
            (coord.0 + offset_1.0, coord.1 + offset_1.1),
            (coord.0 + offset_2.0, coord.1 + offset_2.1)
        ]
    );
}

fn parse(i: &str) -> (Coord, Map, TileTypes) {
    use TileType::*;

    let mut map: Map = BTreeMap::new();
    let mut tile_types: TileTypes = BTreeMap::new();

    let mut start_pos: Coord = (0, 0);
    let _ = i
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| {
                    let coord = (x as isize, y as isize);
                    match ch {
                        '|' => {
                            add_tile(&mut map, coord, (0, -1), (0, 1));
                            tile_types.insert(coord, Vertical);
                        }
                        '-' => {
                            add_tile(&mut map, coord, (-1, 0), (1, 0));
                            tile_types.insert(coord, Horizontal);
                        }
                        'L' => {
                            add_tile(&mut map, coord, (0, -1), (1, 0));
                            tile_types.insert(coord, CornerN);
                        }
                        'J' => {
                            add_tile(&mut map, coord, (0, -1), (-1, 0));
                            tile_types.insert(coord, CornerN);
                        }
                        '7' => {
                            add_tile(&mut map, coord, (0, 1), (-1, 0));
                            tile_types.insert(coord, CornerS);
                        }
                        'F' => {
                            add_tile(&mut map, coord, (0, 1), (1, 0));
                            tile_types.insert(coord, CornerS);
                        }
                        '.' => (),
                        'S' => {
                            start_pos = (x as isize, y as isize);
                        }
                        c => panic!("Invalid character in map {}", c),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sc: Vec<Coord> = Vec::new();

    let _ = map
        .iter()
        .map(|(coord, connections)| {
            if connections.contains(&start_pos) {
                sc.push(*coord);
            }
        })
        .collect::<Vec<_>>();

    if sc.contains(&(start_pos.0 - 1, start_pos.1)) && sc.contains(&(start_pos.0 + 1, start_pos.1)) {
        tile_types.insert(start_pos, Horizontal);
    } else if
        sc.contains(&(start_pos.0, start_pos.1 - 1)) &&
        sc.contains(&(start_pos.0, start_pos.1 + 1))
    {
        tile_types.insert(start_pos, Vertical);
    } else if
        sc.contains(&(start_pos.0, start_pos.1 - 1)) &&
        !sc.contains(&(start_pos.0, start_pos.1 + 1))
    {
        tile_types.insert(start_pos, CornerN);
    } else if
        sc.contains(&(start_pos.0, start_pos.1 + 1)) &&
        !sc.contains(&(start_pos.0, start_pos.1 - 1))
    {
        tile_types.insert(start_pos, CornerS);
    } else {
        panic!("Failed to determine starting tile type");
    }

    map.insert(start_pos, sc);
    (start_pos, map, tile_types)
}

pub fn process(input: &str) -> String {
    use TileType::*;

    let (start_pos, map, tile_types) = parse(input);
    let mut pos = start_pos;
    let mut prev = start_pos;
    let mut path: BTreeSet<Coord> = BTreeSet::new();

    loop {
        let Some(new) = map
            .get(&pos)
            .unwrap()
            .iter()
            .find(|connection| connection != &&prev) else {
            panic!("No direction found");
        };
        path.insert(*new);
        prev = pos;
        pos = *new;
        if pos == start_pos {
            break;
        }
    }
    let min_x = path
        .iter()
        .min_by_key(|k| k.0)
        .unwrap().0;
    let max_x = path
        .iter()
        .max_by_key(|k| k.0)
        .unwrap().0;
    let min_y = path
        .iter()
        .min_by_key(|k| k.1)
        .unwrap().1;
    let max_y = path
        .iter()
        .max_by_key(|k| k.1)
        .unwrap().1;

    (min_y..=max_y)
        .map(|y| {
            let cnt = (min_x..=max_x).fold((0, false, Vertical), |(count, inside, last_cnr), x| {
                if path.contains(&(x, y)) {
                    match tile_types.get(&(x, y)) {
                        Some(Horizontal) => (count, inside, last_cnr),
                        Some(Vertical) => (count, !inside, Vertical),
                        Some(&cnr) if last_cnr == Vertical => (count, inside, cnr),
                        Some(&cnr) if cnr == last_cnr => (count, inside, cnr),
                        Some(_) => (count, !inside, last_cnr),
                        None => panic!("Unknown tile type"),
                    }
                } else if inside {
                    (count + 1, inside, Vertical)
                } else {
                    (count, inside, Vertical)
                }
            }).0;
            cnt
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input =
            "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!("4", process(input));
    }

    #[test]
    fn test_process_2() {
        let input =
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!("8", process(input));
    }

    #[test]
    fn test_process_3() {
        let input =
            "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!("10", process(input));
    }

    #[test]
    fn test_process_input() {
        let input = include_str!("./input-1.txt");
        assert_eq!("357", process(input));
    }
}
