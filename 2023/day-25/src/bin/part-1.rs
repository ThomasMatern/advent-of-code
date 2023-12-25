#![allow(unused_variables, unused_imports, dead_code)]

use std::collections::HashMap;

use nom::{
    IResult,
    Parser,
    character::complete::{ line_ending, alpha1 },
    multi::separated_list1,
    sequence::separated_pair,
};
use nom_supreme::tag::complete::tag;

use rustworkx_core::{
    self,
    petgraph::graph::{ UnGraph, NodeIndex },
    connectivity::stoer_wagner_min_cut,
};

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

fn parse(i: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
    separated_list1(
        line_ending,
        separated_pair(alpha1, tag(": "), separated_list1(tag(" "), alpha1))
    ).parse(i)
}

pub fn process(input: &str) -> String {
    let (_, connections) = parse(input).unwrap();

    let mut nodes: HashMap<&str, NodeIndex> = HashMap::new();
    let mut graph: UnGraph<u32, ()> = UnGraph::new_undirected();

    for (src, dsts) in connections {
        let src_node = *nodes.entry(src).or_insert_with(|| graph.add_node(1));
        for dst in dsts {
            let dst_node = *nodes.entry(dst).or_insert_with(|| graph.add_node(1));
            graph.add_edge(src_node, dst_node, ());
        }
    }
    let min_cut_res = stoer_wagner_min_cut(&graph, |_| Ok::<i32, ()>(1));

    min_cut_res
        .unwrap()
        .map(|(_, subset)| (subset.len() * (nodes.len() - subset.len())).to_string())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input =
            "\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
        assert_eq!("54", process(input));
    }
}
