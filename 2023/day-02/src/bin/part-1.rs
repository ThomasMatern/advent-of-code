use nom::{IResult, branch::alt, sequence::tuple, character::complete::{digit1, space0}, multi::separated_list0, bytes::complete::tag};

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct CubeSet<'a> {
    colour: &'a str,
    count: usize,
}

#[derive(Debug)]
struct Round<'a> {
    sets: Vec<CubeSet<'a>>,
}

#[derive(Debug)]
struct Game<'a> {
    id: usize,
    rounds: Vec<Round<'a>>,
}

fn parse_set(i: &str) -> IResult<&str, CubeSet> {
    let (i, (_, count, _, colour)) = 
        tuple((
            space0, 
            digit1,
            space0, 
            alt((tag("red"), tag("blue"), tag("green")))
        ))(i)?;
    let count = count.parse::<usize>().expect("Bad count");
    Ok((i, CubeSet{colour, count}))
}

fn parse_round(i: &str) -> IResult<&str, Round> {
    let (i, sets) = separated_list0(tag(","), parse_set)(i)?;
    Ok((i, Round {sets}))
}

fn parse_game(i: &str) -> IResult<&str, Game> {
    let (i, (_, id,_)) = tuple((tag("Game "), digit1, tag(":")))(i)?;
    let (i, rounds) = separated_list0(tag(";"), parse_round)(i)?;
    let id = id.parse::<usize>().expect("Bad game-id");
    Ok((i, Game {id, rounds}))
}

fn parse_games(i: &str) -> Vec<Game> {
    i.lines()
     .map(|line| {
        parse_game(line).expect("Bad line").1
     })
     .collect()
}

fn is_valid_game(game: &Game) -> bool {
    game.rounds
        .iter()
        .all(|round| {
            round.sets
                .iter()
                .all(|set| {
                    match set {
                        CubeSet{colour:"red", count} => *count <= 12,
                        CubeSet{colour:"green", count} => *count <= 13,
                        CubeSet{colour:"blue", count} => *count <= 14,
                        _ => panic!("Bad colour")
                    }
                })
        })

}

pub fn process(input: &str) -> String {
    parse_games(input)
        .into_iter()
        .filter(|game| { is_valid_game(game) } )
        .map(|game| { game.id })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8", process(input));
    }
}
