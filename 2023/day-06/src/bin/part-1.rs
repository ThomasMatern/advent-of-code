#![allow(unused_variables,unused_imports,dead_code)]

use nom::{IResult, character::complete::{space1, self, line_ending}, Parser, multi::separated_list1};
use nom_supreme::{tag::complete::tag, ParserExt};

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

struct Race {
    time: u32,
    distance: u32,
}


fn parse_input(i: &str) -> IResult<&str, Vec<Race>> {
    let (i, times) = tag("Time:")
        .precedes(space1)
        .precedes(separated_list1(space1, complete::u32))
        .terminated(line_ending)
        .parse(i)?;
    let (i, distances) = tag("Distance:")
        .precedes(space1)
        .precedes(separated_list1(space1, complete::u32))
        .parse(i)?;
    
    Ok((i, times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| Race {time, distance})
        .collect()))
}

pub fn process(input: &str) -> String {
    let (_, races) = parse_input(input).unwrap();

    races
        .iter()
        .map(|race| {
            (1..race.distance)
                .map(|speed| speed + race.distance / speed)
                .filter(|time| time < &race.time)
                .count() as u32
        })
        .product::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("288", process(input));
    }
}
