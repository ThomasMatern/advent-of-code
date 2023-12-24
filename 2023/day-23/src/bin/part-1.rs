#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

use std::collections::HashMap;
use nom::{
    IResult,
    Parser,
    character::complete::{ one_of, line_ending },
    multi::{ many1, many0 },
    sequence::delimited,
    branch::alt,
    combinator::opt,
    bytes::complete::is_a,
};
use nom_supreme::tag::complete::tag;
use itertools::Itertools;
use glam::IVec2;
use petgraph::{ prelude::*, algo::all_simple_paths };

use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;
type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

fn locate_span(span: Span) -> SpanIVec2 {
    span.map_extra(|_|
        IVec2::new((span.get_column() as i32) - 1, (span.location_line() as i32) - 1)
    )
}

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl From<Direction> for IVec2 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::N => IVec2::new(0, -1),
            Direction::E => IVec2::new(1, 0),
            Direction::S => IVec2::new(0, 1),
            Direction::W => IVec2::new(-1, 0),
        }
    }
}

#[derive(Debug)]
enum TileType {
    Path,
    Slope(Direction),
}

impl TileType {
    fn neighbours(&self) -> Vec<Direction> {
        match self {
            TileType::Path => vec![Direction::N, Direction::E, Direction::S, Direction::W],
            TileType::Slope(dir) => vec![*dir],
        }
    }
}

#[derive(Debug)]
struct Tile<'a> {
    tt: TileType,
    span: SpanIVec2<'a>,
}

impl<'a> Tile<'a> {
    fn from_span(tt: TileType, span: Span<'a>) -> Tile<'a> {
        Tile { tt, span: locate_span(span) }
    }

    fn neighbours(&self) -> Vec<IVec2> {
        let pos = self.span.extra;
        self.tt
            .neighbours()
            .iter()
            .map(|dir| pos + IVec2::from(*dir))
            .collect_vec()
    }
}

type Map<'a> = Vec<Tile<'a>>;

fn parse(i: Span) -> IResult<Span, Map> {
    many1(
        delimited(
            many0(alt((line_ending, tag("#")))),
            alt((
                tag(".").map(|sp| Tile::from_span(TileType::Path, sp)),
                tag("^").map(|sp| Tile::from_span(TileType::Slope(Direction::N), sp)),
                tag(">").map(|sp| Tile::from_span(TileType::Slope(Direction::E), sp)),
                tag("v").map(|sp| Tile::from_span(TileType::Slope(Direction::S), sp)),
                tag("<").map(|sp| Tile::from_span(TileType::Slope(Direction::W), sp)),
            )),
            many0(alt((line_ending, tag("#"))))
        )
    )(i)
}

pub fn process(input: &str) -> String {
    let (_, map) = parse(Span::new(input)).unwrap();
    let mut graph: DiGraph<&Tile, i32> = DiGraph::new();

    let lookup = map
        .iter()
        .map(|tile| (tile.span.extra, graph.add_node(tile)))
        .collect::<HashMap<IVec2, NodeIndex>>();

    let edges = map
        .iter()
        .flat_map(|tile| {
            tile.neighbours()
                .into_iter()
                .filter(|pos| lookup.get(pos).is_some())
                .map(|pos| (lookup[&tile.span.extra], lookup[&pos]))
        })
        .collect_vec();
    edges.iter().for_each(|(a, b)| {
        graph.add_edge(*a, *b, 1);
    });

    let start = map.first().unwrap();
    let goal = map.last().unwrap();

    all_simple_paths::<Vec<_>, _>(
        &graph,
        lookup[&start.span.extra],
        lookup[&goal.span.extra],
        0,
        None
    )
        .max_by_key(|path| path.len())
        .map(|path| path.len() - 1)
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input =
            "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        assert_eq!("94", process(input));
    }
}
