#![allow(unused_variables,unused_imports,dead_code,unused_mut)]

use std::collections::BTreeMap;

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

type Coord = (isize, isize);
type Map = BTreeMap<Coord, Vec<Coord>>;

fn add_tile(m: &mut Map, coord: (usize, usize), offset_1: Coord, offset_2:Coord) {
    m.insert((coord.0 as isize, coord.1 as isize),  
        vec![
            (coord.0 as isize + offset_1.0, coord.1 as isize + offset_1.1),
            (coord.0 as isize + offset_2.0, coord.1 as isize + offset_2.1)]);
}

fn parse(i: &str) -> (Coord, Map) {
    let mut map: Map = BTreeMap::new();

    let mut start_pos: Coord = (0, 0);
    let _ = i
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| match ch {
                    '|' => add_tile(&mut map, (x, y), (0, -1), (0, 1)),
                    '-' => add_tile(&mut map, (x, y), (-1, 0), (1, 0)),
                    'L' => add_tile(&mut map, (x, y), (0, -1), (1, 0)),
                    'J' => add_tile(&mut map, (x, y), (0, -1), (-1, 0)),
                    '7' => add_tile(&mut map, (x, y), (0, 1), (-1, 0)),
                    'F' => add_tile(&mut map, (x, y), (0, 1), (1, 0)),
                    '.' => (),
                    'S' => start_pos = (x as isize, y as isize),
                    c => panic!("Invalid character in map {}", c)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    

    let mut start_connections:Vec<Coord> = Vec::new();

    let _ = map.iter()
        .map(|(coord, connections)| {
            if connections.contains(&start_pos) {
                start_connections.push(coord.clone());
            }
        })
        .collect::<Vec<_>>();
    map.insert(start_pos, start_connections);
    (start_pos, map)
}

pub fn process(input: &str) -> String {
    let (start_pos, map) = parse(input);
    let mut pos = start_pos;
    let mut prev = start_pos;
    let mut count = 0;
    loop {
        let Some(new) = map.get(&pos).unwrap()
            .iter()
            .find(|connection| connection != &&prev)
        else {
            panic!("No direction found");
        };
        count += 1;
        prev = pos;
        pos = *new;
        if pos == start_pos {
            break;
        }
    }
    (count / 2).to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!("4", process(input));
    }

    #[test]
    fn test_process_2() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!("8", process(input));
    }
    #[test]
    fn test_process_3() {
        let input = include_str!("./input-1.txt");
        assert_eq!("6931", process(input));
    }
}
