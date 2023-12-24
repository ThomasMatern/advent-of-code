#![allow(unused_variables,unused_imports,dead_code, unused_mut)]

use core::fmt;

use nom::{IResult, Parser, character::complete::{self, line_ending}, multi::separated_list1, sequence::{separated_pair, tuple}};
use nom_supreme::{tag::complete::tag, ParserExt};
use itertools::Itertools;
use defaultdict::DefaultHashMap;


fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Brick {
    high: Pos,
    low: Pos,
}

impl Brick {
    fn overlaps_2d(&self, other: &Brick) -> bool {
        self.low.x.max(other.low.x) <= self.high.x.min(other.high.x) && self.low.y.max(other.low.y) <= self.high.y.min(other.high.y)
    }

    fn touches_above(&self, other: &Brick) -> bool {
        self.overlaps_2d(other) && self.high.z == other.low.z - 1
    }

    fn drop(&mut self, low_z: i32) {
        self.high.z += low_z - self.low.z;
        self.low.z = low_z;
    }
}

impl fmt::Debug for Brick {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Brick {}/{}/{} -> {}/{}/{}", self.low.x, self.low.y, self.low.z, self.high.x, self.high.y, self.high.z)
    }
}

fn parse_vec(i: &str) -> IResult<&str, Pos> {
    tuple((
        complete::i32.terminated(tag(",")),
        complete::i32.terminated(tag(",")),
        complete::i32)).map(|(x, y, z)| Pos{x, y, z})
        .parse(i)
}

fn parse_brick(i: &str) -> IResult<&str, Brick> {
    separated_pair(
        parse_vec,
        tag("~"),
        parse_vec).map(|(low, high)| Brick{low, high})
        .parse(i)
}


fn parse(i: &str) -> IResult<&str, Vec<Brick>> {
    separated_list1(
        line_ending,
        parse_brick
    ).parse(i)
}

pub fn process(input: &str) -> String {
    let (_, mut bricks) = parse(input).unwrap();
    bricks.sort_by_key(|brick| brick.high.z);

    for my_idx in 0..bricks.len() {
        let low_z = (0..my_idx)
            .filter(|&other_idx| bricks[my_idx].overlaps_2d(&bricks[other_idx]))
            .map(|other_idx| bricks[other_idx].high.z + 1).max().unwrap_or(1);      
        bricks[my_idx].drop(low_z);
    }
    bricks.sort_by_key(|brick| brick.high.z);

    let mut supports: DefaultHashMap<&Brick, Vec<&Brick>> = DefaultHashMap::new();
    let mut is_supported: DefaultHashMap<&Brick, Vec<&Brick>> = DefaultHashMap::new();

    bricks.iter()
        .combinations(2)
        .filter(|lower_upper| lower_upper[0].touches_above(lower_upper[1]))
        .for_each(|lower_upper| {
            let lower = lower_upper[0];
            let upper = lower_upper[1];
            supports.get_mut(&lower).push(upper);
            is_supported.get_mut(&upper).push(lower);
        });

    bricks.iter().filter(|brick| 
            supports[&brick]
                .iter()
                .all(|other| is_supported[&other].len() >= 2)
        )
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        assert_eq!("5", process(input));
    }
}
