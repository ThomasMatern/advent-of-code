#![allow(unused_variables, unused_imports, dead_code, unused_must_use, unused_mut)]

use std::collections::BTreeMap;

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
enum Cell {
    Empty,
    Symbol,
    Number(u32),
}

fn symbol_nearby(map: &BTreeMap<(usize, usize), Cell>, x: usize, y: usize) -> bool {
    for yy in y.max(1) - 1..=y + 1 {
        for xx in x.max(1) - 1..=x + 1 {
            if let Some(Cell::Symbol) = map.get(&(xx, yy)) {
                return true;
            }
        }
    }
    false
}

pub fn process(input: &str) -> String {
    let schematic = input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| {
                    let cell = match ch {
                        '.' => Cell::Empty,
                        c if c.is_ascii_digit() => Cell::Number(c.to_digit(10).unwrap()),
                        _ => Cell::Symbol,
                    };
                    ((x, y), cell)
                })
        })
        .collect::<BTreeMap<(usize, usize), Cell>>();

    let mut sum = 0;

    for y in 0.. {
        match schematic.get(&(0, y)) {
            Some(_) => {
                let mut part_number: Option<(u32, bool)> = None;

                for x in 0.. {
                    match schematic.get(&(x, y)) {
                        Some(cell) => {
                            part_number = match (cell, part_number) {
                                (Cell::Number(digit), Some((value, valid))) =>
                                    Some((
                                        value * 10 + *digit,
                                        valid || symbol_nearby(&schematic, x, y),
                                    )),

                                (_, Some((value, valid))) => {
                                    if valid {
                                        sum += value;
                                    }
                                    None
                                }
                                (Cell::Number(digit), None) =>
                                    Some((*digit, symbol_nearby(&schematic, x, y))),

                                (_, None) => None,
                            };
                        }
                        None => {
                            if let Some((value, valid)) = part_number {
                                if valid {
                                    sum += value;
                                }
                            }
                            break;
                        }
                    }
                }
            }
            None => {
                break;
            }
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input =
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        assert_eq!("4361", process(input));
    }
}
