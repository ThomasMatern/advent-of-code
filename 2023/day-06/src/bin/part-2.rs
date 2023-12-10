#![allow(unused_variables,unused_imports,dead_code)]

use nom::{IResult, character::complete::{space1, line_ending, digit1}, Parser, multi::separated_list1};
use nom_supreme::{tag::complete::tag, ParserExt};
use binary_search::{binary_search, Direction};

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

fn is_winner(race: &Race, speed: u128) -> bool {
    speed + race.distance / speed < race.time
}

pub fn process(input: &str) -> String {
    let (_, race) = parse_input(input).unwrap();

    // time = speed (time button down) + distance / speed
    // dt = 1 - distance / speed^2
    // best: dt = 0 = 1 - distance / speed^2
    // best: 1 = distance / speed^2
    // best: speed = sqrt(distance)

    let best = (race.distance as f64).sqrt() as u128;
    let lower = 
        binary_search((1, ()), (race.distance, ()), 
        |speed| {
            if speed > best || is_winner(&race, speed) 
                { Direction::High(()) } else { Direction::Low(()) }
        });
    let upper = 
        binary_search((1, ()), (race.distance, ()), 
        |speed| {
            if speed < best || is_winner(&race, speed) 
                { Direction::Low(()) } else { Direction::High(()) }
        });
    (upper.0.0 - lower.1.0 + 1).to_string()
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
