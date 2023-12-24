// #![allow(unused_variables, unused_imports, dead_code, unused_mut)]

use std::collections::{ HashMap, VecDeque, HashSet };
use nom::{
    IResult,
    Parser,
    character::complete::line_ending,
    multi::{ many1, many0 },
    sequence::delimited,
    branch::alt,
};
use nom_supreme::tag::complete::tag;
use itertools::Itertools;
use glam::IVec2;
use petgraph::{ prelude::*, algo::all_simple_paths, dot::{ Dot, Config } };

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

#[derive(Debug, Hash, Eq, PartialEq)]
enum TileType {
    Path,
}

impl TileType {
    fn neighbours(&self) -> Vec<Direction> {
        match self {
            TileType::Path => vec![Direction::N, Direction::E, Direction::S, Direction::W],
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
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
type TileMap<'a> = HashMap<&'a IVec2, &'a Tile<'a>>;

fn parse(i: Span) -> IResult<Span, Map> {
    many1(
        delimited(
            many0(alt((line_ending, tag("#")))),
            alt((
                tag(".").map(|sp| Tile::from_span(TileType::Path, sp)),
                tag("^").map(|sp| Tile::from_span(TileType::Path, sp)),
                tag(">").map(|sp| Tile::from_span(TileType::Path, sp)),
                tag("v").map(|sp| Tile::from_span(TileType::Path, sp)),
                tag("<").map(|sp| Tile::from_span(TileType::Path, sp)),
            )),
            many0(alt((line_ending, tag("#"))))
        )
    )(i)
}

fn valid_neighbours(map: &TileMap, tile: &Tile) -> Vec<IVec2> {
    tile.neighbours()
        .iter()
        .filter(|t| map.contains_key(t))
        .cloned()
        .collect_vec()
}

pub fn process(input: &str) -> String {
    let (_, map) = parse(Span::new(input)).unwrap();
    let mut graph: DiGraph<&Tile, i32> = DiGraph::new();
    let start = map.first().unwrap();
    let goal = map.last().unwrap();

    let tile_lookup = map
        .iter()
        .map(|tile| (&tile.span.extra, tile))
        .collect::<TileMap>();

    let node_id_lookup = map
        .iter()
        .filter(
            |&tile|
                tile == start || tile == goal || valid_neighbours(&tile_lookup, tile).len() >= 3
        )
        .map(|tile| (tile.span.extra, graph.add_node(tile)))
        .collect::<HashMap<IVec2, NodeIndex>>();

    let mut edges: HashMap<(NodeIndex, NodeIndex), i32> = HashMap::new();

    for pos in node_id_lookup.keys() {
        let mut queue: VecDeque<(IVec2, i32)> = VecDeque::new();
        let mut seen: HashSet<IVec2> = HashSet::new();
        queue.push_back((*pos, 0));
        seen.insert(*pos);

        while let Some((other_pos, weight)) = queue.pop_front() {
            if weight > 0 && node_id_lookup.contains_key(&other_pos) {
                let pos_idx = node_id_lookup[&pos];
                let other_idx = node_id_lookup[&other_pos];
                graph.add_edge(pos_idx, other_idx, weight);
                edges.insert((pos_idx, other_idx), weight);
                continue;
            }
            for nb in valid_neighbours(&tile_lookup, tile_lookup[&other_pos]) {
                if !seen.contains(&nb) {
                    queue.push_back((nb, weight + 1));
                    seen.insert(nb);
                }
            }
        }
    }

    dbg!(Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    all_simple_paths::<Vec<_>, _>(
        &graph,
        node_id_lookup[&start.span.extra],
        node_id_lookup[&goal.span.extra],
        0,
        None
    )
        .map(|path|
            path
                .iter()
                .tuple_windows()
                .map(|(a, b)| edges[&(*a, *b)])
                .sum::<i32>()
        )
        .max()
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
        assert_eq!("154", process(input));
    }
}
