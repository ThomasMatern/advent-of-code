// #![allow(unused_variables, unused_imports, dead_code, unused_must_use)]

use std::{ ops::Range, collections::BTreeMap };
use itertools::Itertools;

use nom::{
    IResult,
    sequence::tuple,
    character::complete::{ self, alpha1, space1, line_ending },
    multi::{ many0, fold_many1, separated_list0 },
    bytes::complete::tag,
};

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct MapRange {
    src_start: i64,
    src_end: i64,
    offset: i64,
}

impl MapRange {
    fn parse(i: &str) -> IResult<&str, MapRange> {
        let (i, (_, dst_start, _, src_start, _, size)) = tuple((
            line_ending,
            complete::i64,
            space1,
            complete::i64,
            space1,
            complete::i64,
        ))(i)?;
        Ok((i, MapRange { src_start, src_end: src_start + size, offset: dst_start - src_start }))
    }

    fn map_range(&self, range: &Range<i64>) -> (Range<i64>, Range<i64>) {
        // returns the mapped range and any remainder
        let start = self.src_start.max(range.start);
        let end = self.src_end.min(range.end);
        (start + self.offset..end + self.offset, end.max(range.start)..range.end)
    }
}

#[derive(Debug)]
struct Map {
    ranges: BTreeMap<i64, MapRange>,
}

impl Map {
    fn parse(i: &str) -> IResult<&str, Map> {
        let (i, _) = tuple((many0(line_ending), alpha1, tag("-to-"), alpha1, tag(" map:")))(i)?;

        let (i, ranges) = fold_many1(
            MapRange::parse,
            BTreeMap::new,
            |mut acc: BTreeMap<_, _>, item| {
                acc.insert(item.src_start, item);
                acc
            }
        )(i)?;
        Ok((i, Map { ranges }))
    }

    fn map_range(&self, from: &Range<i64>) -> Vec<Range<i64>> {
        let mut result: Vec<Range<i64>> = Vec::new();
        let mut remainder = from.clone();
        for range in self.ranges.values() {
            if range.src_start > remainder.start {
                // map stuff before this ranges
                result.push(from.start..range.src_start);
                remainder = range.src_start..remainder.end;
            }

            let (mapped, rem) = range.map_range(&remainder);
            remainder = rem;
            if !mapped.is_empty() {
                result.push(mapped);
            }
            if remainder.is_empty() {
                break;
            }
        }
        if !remainder.is_empty() {
            // map stuff after our ranges
            result.push(remainder);
        }
        result
    }

    fn map_ranges(&self, from: Vec<Range<i64>>) -> Vec<Range<i64>> {
        from.iter()
            .flat_map(|r| { self.map_range(r) })
            .collect_vec()
    }
}

fn parse_seeds(i: &str) -> IResult<&str, Vec<Range<i64>>> {
    let (i, (_, _, values)) = tuple((
        tag("seeds:"),
        space1,
        separated_list0(space1, complete::i64),
    ))(i)?;
    let seeds = values
        .chunks(2)
        .map(|chunk| Range { start: chunk[0], end: chunk[0] + chunk[1] })
        .collect::<Vec<_>>();
    Ok((i, seeds))
}

pub fn process(input: &str) -> String {
    let (i, seeds) = parse_seeds(input).unwrap();
    let (_i, maps) = fold_many1(Map::parse, Vec::new, |mut acc: Vec<_>, item| {
        acc.push(item);
        acc
    })(i).unwrap();

    maps.iter()
        .fold(seeds, |acc, map| { map.map_ranges(acc) })
        .iter()
        .map(|r| r.start)
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input =
            "seeds: 79 14 55 13

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
        assert_eq!("46", process(input));
    }
}
