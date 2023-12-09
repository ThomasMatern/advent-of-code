#![allow(unused_variables,unused_imports,dead_code,unused_must_use,unused_mut)]

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
    Gear,
    Number(u32)
}


fn get_number_at(map: &BTreeMap<(usize, usize),Cell>, x: usize, y: usize) -> Option<(usize, usize, u32)> {
    match map.get(&(x, y)) {
        Some(Cell::Number(_)) => {
            let mut idx = x;
            while idx > 0 && matches!(map.get(&(idx-1, y)), Some(Cell::Number(_))) {
                idx -= 1;
            }
            let mut num = 0;
            let mut num_idx = idx;
            loop {
                if let Some(Cell::Number(d)) = map.get(&(num_idx, y)) {
                    num_idx += 1;
                    num = num * 10 + d;
                } else {
                    break
                }
            };
            Some((idx, y, num))
        }
        _ => None,
    }

}

fn handle_gear(map: &BTreeMap<(usize, usize),Cell>, x: usize, y: usize) -> u32 {
    let mut numbers:BTreeMap<(usize, usize),u32> = BTreeMap::new();

    for yy in y.max(1)-1..=y+1 {
        for xx in x.max(1)-1..=x+1 {
            match get_number_at(map, xx, yy)
            {
                None => (),
                Some((number_x, number_y, number)) =>
                {
                    numbers.insert((number_x, number_y), number);
                }
            }
        }
    }
    if numbers.len() == 2 {
        return numbers
            .iter()
            .map(|(_, num)| *num)
            .product::<u32>()
    }

    0
}

pub fn process(input: &str) -> String {
    let schematic = input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line
                .chars()
                .enumerate()
                .map(move |(x, ch)| {
                    let cell = match ch {
                        '.' => Cell::Empty,
                        '*' => Cell::Gear,
                        c if c.is_digit(10) => Cell::Number(c.to_digit(10).unwrap()),
                        _ => Cell::Symbol
                    };
                    ((x, y), cell)
                })
        })
        .collect::<BTreeMap<(usize, usize), Cell>>();

    schematic
        .iter()
        .filter(|(_, cell)| {
            matches!(cell, Cell::Gear)
        })
        .map(|((x, y), _)| {
            handle_gear(&schematic, *x, *y)
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "467..114..
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
        assert_eq!("467835", process(input));
    }
}
