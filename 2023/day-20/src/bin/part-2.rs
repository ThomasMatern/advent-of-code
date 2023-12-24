#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

use std::collections::{ BTreeMap, VecDeque, HashMap };

use nom::{
    IResult,
    Parser,
    character::complete::{ line_ending, alpha1 },
    multi::separated_list1,
    branch::alt,
    sequence::tuple,
};
use nom_supreme::{ tag::complete::tag, ParserExt };
use itertools::Itertools;
use num::integer::lcm;

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

#[derive(Debug)]
enum Module {
    FF,
    CJ,
    BC,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Low,
    High,
}

#[derive(Debug)]
struct ModuleState<'a> {
    inputs: BTreeMap<&'a str, State>,
    output: State,
}

#[derive(Debug)]
struct ModuleConnections<'a> {
    module: Module,
    connections: Vec<&'a str>,
}

fn parse(i: &str) -> IResult<&str, Vec<(&str, ModuleConnections)>> {
    separated_list1(
        line_ending,
        tuple((
            alt((
                alpha1.preceded_by(tag("%")).map(|name| (name, Module::FF)),
                alpha1.preceded_by(tag("&")).map(|name| (name, Module::CJ)),
                tag("broadcaster").map(|name| (name, Module::BC)),
            )).terminated(tag(" -> ")),
            separated_list1(tag(", "), alpha1),
        )).map(|((name, module), connections)| (name, ModuleConnections { module, connections }))
    ).parse(i)
}

pub fn process(input: &str) -> String {
    use State::*;
    use Module::*;

    let (_, modules) = parse(input).unwrap();
    let mut map: HashMap<&str, ModuleConnections> = HashMap::from_iter(modules);
    let mut state: HashMap<&str, ModuleState> = HashMap::new();

    // Add rx as a module so we can get its inputs
    map.insert("rx", ModuleConnections { module: FF, connections: vec![] });

    for module_name in map.keys() {
        state.insert(module_name, ModuleState { inputs: BTreeMap::new(), output: Low });
    }

    for (module_name, module) in &map {
        for connection in &module.connections {
            if let Some(output) = state.get_mut(connection) {
                output.inputs.insert(module_name, Low);
            }
        }
    }

    // Assumption: the number is too big to iterate to.
    // The dataset shows that rx has one input (CJ). We look for the cycles of the inputs of that module.

    // Get the input of rx
    let rx_inputs = state[&"rx"].inputs.keys().collect_vec();

    // Find the parents of that module
    let rx_parent_inputs = rx_inputs
        .iter()
        .flat_map(|input| { state[*input].inputs.keys().copied() })
        .collect_vec();

    // Keep track of loop counts
    let mut rx_parent_loops: HashMap<&str, isize> = HashMap::new();
    let mut pulses: VecDeque<(&str, State)> = VecDeque::new();
    let mut broadcast_counter: isize = 0;
    loop {
        if pulses.is_empty() {
            broadcast_counter += 1;
            if broadcast_counter % 1000000 == 0 {
                println!("{}", broadcast_counter);
            }
            pulses.push_back(("broadcaster", Low));
        }
        if rx_parent_inputs.len() == rx_parent_loops.len() {
            // Done - we have a number for all inputs
            break;
        }

        while let Some((from_module_name, pulse)) = pulses.pop_front() {
            for &to_module_name in &map[&from_module_name].connections {
                if pulse == Low {
                    for parent_input in &rx_parent_inputs {
                        if
                            !rx_parent_loops.contains_key(parent_input) &&
                            parent_input == &to_module_name
                        {
                            // Low pulse on a module that we have not seen before.
                            rx_parent_loops.insert(parent_input, broadcast_counter);
                            println!("{}: {}", to_module_name, broadcast_counter);
                        }
                    }
                }
                if let Some(to_module) = &map.get(to_module_name) {
                    let to_module_state = state.get_mut(&to_module_name).unwrap();
                    to_module_state.inputs.insert(from_module_name, pulse);
                    match to_module.module {
                        FF => if pulse == Low {
                            if to_module_state.output == Low {
                                to_module_state.output = High;
                                pulses.push_back((&to_module_name, High));
                            } else {
                                to_module_state.output = Low;
                                pulses.push_back((&to_module_name, Low));
                            }
                        }
                        CJ => {
                            if to_module_state.inputs.iter().all(|(_, &s)| s == High) {
                                pulses.push_back((&to_module_name, Low));
                            } else {
                                pulses.push_back((&to_module_name, High));
                            }
                        }
                        BC => panic!("cannot send to broadcast module"),
                    }
                }
            }
        }
    }

    rx_parent_loops
        .iter()
        .fold(1isize, |acc, (_, factor)| lcm(acc, *factor))
        .to_string()
}
