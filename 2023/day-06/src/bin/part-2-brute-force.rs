#![allow(unused_variables,unused_imports,dead_code)]

use nom::{IResult, character::complete::{space1, line_ending, digit1}, Parser, multi::separated_list1};
use nom_supreme::{tag::complete::tag, ParserExt};

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct Race {
    time: u128,
    distance: u128,
}


fn parse_input(i: &str) -> IResult<&str, Race> {
    let (i, time) = tag("Time:")
        .precedes(space1)
        .precedes(separated_list1(space1, digit1))
        .terminated(line_ending)
        .parse(i)?;
    let (i, distance) = tag("Distance:")
        .precedes(space1)
        .precedes(separated_list1(space1, digit1))
        .parse(i)?;

    let time: u128 = time.join("").parse().unwrap();
    let distance: u128 = distance.join("").parse().unwrap();
    Ok((i, Race{time, distance}))
}

pub fn process(input: &str) -> String {
    let (_, race) = parse_input(input).unwrap();

    (1..race.time)
        .map(|time| (race.time - time) * time)
        .filter(|distance| distance > &race.distance)
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("71503", process(input));
    }
}
