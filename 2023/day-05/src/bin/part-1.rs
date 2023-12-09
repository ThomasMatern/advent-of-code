#![allow(unused_variables, unused_imports, dead_code, unused_must_use)]

use nom::{IResult, sequence::tuple, character::complete::{self, alpha1, digit1, space0, space1, line_ending}, multi::{many0, fold_many1, separated_list0}, bytes::complete::{tag, take_till}};

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct MapRange {
    dst: i64,
    src: i64,
    size: i64
}

impl MapRange {
    fn parse(i: &str) -> IResult<&str, MapRange> {
        let (i, (_, dst, _, src, _, size)) = tuple((line_ending, complete::i64, space1, complete::i64, space1, complete::i64))(i)?;
        Ok((i, MapRange{dst, src, size}))
    }

    fn map(&self, from: i64) -> Option<i64> {
        if from >= self.src && from < self.src + self.size {
            return Some(from - self.src + self.dst)
        }
        None
    }
}

#[derive(Debug)]
struct Map<'a> {
    from: &'a str,
    to: &'a str,
    ranges: Vec<MapRange>,
}

impl Map<'_> {
    fn parse(i: &str) -> IResult<&str, Map> {
        let ranges: Vec<MapRange> = Vec::new();
        let (i, (_, from, _, to, _)) = tuple(
            (many0(line_ending),
            alpha1, tag("-to-"), alpha1,
            tag(" map:")))(i)?;
  
        let (i, ranges) = fold_many1(
                MapRange::parse,
                Vec::new,
                |mut acc: Vec<_>, item| {
                  acc.push(item);
                  acc
                }
              )(i)?;
        Ok((i, Map {from, to, ranges}))
    }

    fn map_value(&self, from: i64) -> i64 {
        self.ranges
            .iter()
            .filter_map(|range| range.map(from))
            .next().unwrap_or(from)
    }

}

fn parse_seeds(i: &str) -> IResult<&str, Vec<i64>> {
    let (i, (_, _, seeds)) = tuple((tag("seeds:"), space1, separated_list0(space1, complete::i64)))(i)?;
    Ok((i, seeds))
}

pub fn process(input: &str) -> String {
    let (i, seeds) = parse_seeds(input).unwrap();
    let (i, maps) = fold_many1(
        Map::parse,
        Vec::new,
        |mut acc: Vec<_>, item| {
            acc.push(item);
            acc
          }
        )(i).unwrap();

    maps
        .iter()
        .fold(
            seeds,
            |acc, map| {
                acc.iter()
                    .map(|from| map.map_value(*from))
                    .collect::<Vec<_>>()
            }
        ).iter().min().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!("35", process(input));
    }
}
