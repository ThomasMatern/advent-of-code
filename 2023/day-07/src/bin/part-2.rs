#![allow(unused_variables,unused_imports,dead_code, unused_mut)]

use nom::{IResult, sequence::separated_pair, character::complete::{space1, self, alphanumeric1}};
use itertools::Itertools;

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
struct Hand<'a> {
    cards: &'a str,
    bid: u32,
}

impl<'a> Hand<'a> {
    fn value(&self) -> u32 {
        let ranked_cards = "AKQT98765432J";
        let ranked_cards_without_joker = "AKQT98765432";
        let num_jokers = self.cards.matches('J').count();

        let mut dist = ranked_cards_without_joker
            .chars()
            .map(|ch| self.cards.matches(ch).count())
            .filter(|count| *count > 0)
            .sorted()
            .rev()
            .collect::<Vec<_>>();

        if num_jokers == 5 {
            dist = vec![5]
        } else {
            dist[0] = (dist[0] + num_jokers).min(5)
        };

        let mut rank:u32 = match dist[..] {
            [5, ..] => 0,
            [4, ..] => 1,
            [3, 2, ..] => 2,
            [3, ..] => 3,
            [2, 2, ..] => 4,
            [2, ..] => 5,
            _ => 6,
        };

        for ch in self.cards.chars() {
            rank = ranked_cards.len() as u32 * rank + ranked_cards.find(ch).unwrap() as u32;
        }

        rank
    }
}

fn parse(i: &str) -> IResult<&str, Hand> {
    let (i, (cards, bid)) = separated_pair(alphanumeric1, space1, complete::u32)(i)?;
    Ok((i, Hand{cards, bid} ))
}

pub fn process(i: &str) -> String {
    let cards = i
        .lines()
        .map(|line| parse(line).unwrap().1)
        .collect::<Vec<_>>();

    cards
        .iter()
        .sorted_by_key(|hand| hand.value())
        .rev()
        .enumerate()
        .map(|(rank, hand)| (rank as u32 + 1) * hand.bid)
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("5905", process(input));
    }
}
