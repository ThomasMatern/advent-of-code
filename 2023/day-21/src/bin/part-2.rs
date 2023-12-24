#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

use std::collections::{ BTreeSet, HashSet };

use nom::{
    IResult,
    Parser,
    character::complete::{ self, line_ending },
    multi::{ separated_list1, many1 },
    branch::alt,
};
use itertools::Itertools;
use polyfit_rs::polyfit_rs::polyfit;

fn main() {
    let input = include_str!("./input-1.txt");
    println!("{}", process(input, 26501365));
}

#[derive(Debug, PartialEq)]
enum Tile {
    Garden,
    Rocks,
    Start,
}

type Coord = (i32, i32);
type Garden = BTreeSet<Coord>;

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

fn process_step(garden: &Garden, positions: &HashSet<Coord>, size: Coord) -> HashSet<Coord> {
    let mut new_positions: HashSet<Coord> = HashSet::new();

    for pos in positions {
        for (dx, dy) in [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
        ] {
            let new_coord = (pos.0 + dx, pos.1 + dy);
            let map_coord = (new_coord.0.rem_euclid(size.0), new_coord.1.rem_euclid(size.1));

            if !garden.contains(&map_coord) {
                // Rock
                continue;
            }
            new_positions.insert(new_coord);
        }
    }

    new_positions
}

pub fn process(input: &str, num_steps: i32) -> String {
    let (_, garden_vec) = parse(input).unwrap();
    let width = garden_vec[0].len() as i32;
    let height = garden_vec.len() as i32;
    let size: Coord = (width, height);

    let mut garden: Garden = BTreeSet::new();
    let mut positions: HashSet<Coord> = HashSet::new();
    let mut start: Coord = (0, 0);

    garden_vec
        .iter()
        .enumerate()
        .for_each(|(y, row)|
            row
                .iter()
                .enumerate()
                .for_each(|(x, t)| {
                    match *t {
                        Tile::Start => {
                            start = (x as i32, y as i32);
                            garden.insert(start);
                            positions.insert(start);
                        }
                        Tile::Garden => {
                            garden.insert((x as i32, y as i32));
                        }
                        _ => (),
                    }
                })
        );

    // This is too hard to count - but the input is pretty special:
    // - the grid is quadratic, the width and height are odd (131)
    // - there is a horizontal and a vertical empty line from S
    // - there is a diamond shape, ensuring symmetric flood-fill
    // - the diamond is reached after 65 steps
    // - after those 65 steps, another, bigger diamond is reached after another 131 steps
    // - the total step count reaches a full diamond at the end
    // just calculating the first few diamond-layers and then fitting a quadratic eq seems to work.

    if width != height {
        panic!("Bad input data");
    }

    let steps_center = width / 2;
    let max_loops = 7;
    let max_steps = max_loops * width + steps_center;

    let width_diamonds: i64 = ((num_steps - steps_center) / width) as i64;
    let mut polyfit_x_values: Vec<f64> = vec![];
    let mut polyfit_y_values: Vec<f64> = vec![];
    let mut result: i64 = 0;

    for step in 0..=max_steps {
        if step % width == steps_center {
            polyfit_x_values.push(polyfit_x_values.len() as f64);
            polyfit_y_values.push(positions.len() as f64);

            if polyfit_x_values.len() >= 3 {
                // need at least 3 values
                if let Ok(fit) = polyfit(&polyfit_x_values, &polyfit_y_values, 2) {
                    let fit = fit
                        .iter()
                        .map(|v| (*v).round() as i64)
                        .collect_vec();
                    result = fit
                        .iter()
                        .enumerate()
                        .map(|(pow, val)| val * width_diamonds.pow(pow as u32))
                        .sum::<i64>();
                    println!("Loop: {}, estimated value: {}", polyfit_x_values.len(), result);
                }
            }
        }
        positions = process_step(&garden, &positions, size);
    }

    result.to_string()
}
