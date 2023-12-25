#![allow(unused_variables, unused_imports, dead_code)]

use nom::{
    IResult,
    Parser,
    character::complete::{ self, alphanumeric1, line_ending },
    multi::{ separated_list1, count },
    sequence::{ separated_pair, tuple },
};
use nom_supreme::{ tag::complete::tag, ParserExt };
use itertools::Itertools;
use rayon::prelude::*;
use indicatif::{ ProgressIterator, ParallelProgressIterator };
use glam::{ I64Vec2, f64::DVec2 };

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input, (200000000000000.0, 400000000000000.0));
    dbg!(output);
}

struct HailStone {
    location: DVec2,
    direction: DVec2,
}

fn parse(i: &str) -> IResult<&str, Vec<HailStone>> {
    separated_list1(
        line_ending,
        separated_pair(
            tuple((
                complete::i64.terminated(tag(", ")),
                complete::i64.terminated(tag(", ")),
                complete::i64,
            )).map(|(x, y, _)| DVec2::new(x as f64, y as f64)),
            tag(" @ "),
            tuple((
                complete::i64.terminated(tag(", ")),
                complete::i64.terminated(tag(", ")),
                complete::i64,
            )).map(|(x, y, _)| DVec2::new(x as f64, y as f64))
        ).map(|(location, direction)| HailStone { location, direction })
    ).parse(i)
}

pub fn process(input: &str, limits: (f64, f64)) -> String {
    let (_, hailstones) = parse(input).unwrap();

    let mut cnt = 0;
    for cmb in (0..hailstones.len()).combinations(2) {
        let a = &hailstones[cmb[0]];
        let b = &hailstones[cmb[1]];

        if a.location == b.location {
            cnt += 1;
            continue;
        }

        if a.direction == b.direction {
            continue;
        }

        let dx = b.location.x - a.location.x;
        let dy = b.location.y - a.location.y;
        let det = b.direction.x * a.direction.y - b.direction.y * a.direction.x;

        if det == 0.0 {
            continue;
        }
        let u = (dy * b.direction.x - dx * b.direction.y) / det;
        let v = (dy * a.direction.x - dx * a.direction.y) / det;

        if u <= 0.0 || v <= 0.0 {
            continue;
        }

        let pa = a.location + u * a.direction;
        let pb = b.location + v * b.direction;

        if pa.x >= limits.0 && pa.x <= limits.1 && pa.y >= limits.0 && pa.y <= limits.1 {
            cnt += 1;
        }

    }

    cnt.to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input =
            "\
19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        assert_eq!("2", process(input, (7.0, 27.0)));
    }
}
