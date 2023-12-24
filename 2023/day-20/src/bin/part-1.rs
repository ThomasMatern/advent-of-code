#![allow(unused_variables, unused_imports, dead_code, unused_mut)]

use std::collections::{ BTreeMap, VecDeque };

use nom::{
    IResult,
    Parser,
    character::complete::{ line_ending, alpha1 },
    multi::separated_list1,
    branch::alt,
    sequence::tuple,
};
use nom_supreme::{ tag::complete::tag, ParserExt };

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
    let map: BTreeMap<&str, ModuleConnections> = BTreeMap::from_iter(modules);
    let mut state: BTreeMap<&str, ModuleState> = BTreeMap::new();

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

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    let mut pulses: VecDeque<(&str, State)> = VecDeque::new();
    let mut broadcast_counter = 0;
    loop {
        if pulses.is_empty() {
            dbg!(broadcast_counter += 1);
            if broadcast_counter > 1000 {
                break;
            }

            pulses.push_back(("broadcaster", Low));
            low_pulses += 1;
        }

        let mut next_pulses: VecDeque<(&str, State)> = VecDeque::new();
        for (from_module_name, pulse) in pulses {
            for &to_module_name in &map[&from_module_name].connections {
                if pulse == Low {
                    low_pulses += 1;
                } else {
                    high_pulses += 1;
                }
                println!("{} -{:?}- -> {}", &from_module_name, pulse, &to_module_name);
                if let Some(to_module) = &map.get(to_module_name) {
                    let to_module_state = state.get_mut(&to_module_name).unwrap();
                    to_module_state.inputs.insert(from_module_name, pulse);
                    match to_module.module {
                        FF => if pulse == Low {
                            if to_module_state.output == Low {
                                to_module_state.output = High;
                                next_pulses.push_back((&to_module_name, High));
                            } else {
                                to_module_state.output = Low;
                                next_pulses.push_back((&to_module_name, Low));
                            }
                        }
                        CJ => {
                            if to_module_state.inputs.iter().all(|(_, &s)| s == High) {
                                next_pulses.push_back((&to_module_name, Low));
                            } else {
                                next_pulses.push_back((&to_module_name, High));
                            }
                        }
                        BC => panic!("cannot send to broadcast module"),
                    }
                }
            }
        }
        pulses = next_pulses;
    }

    dbg!(low_pulses, high_pulses);
    (low_pulses * high_pulses).to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process_1() {
        let input = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        assert_eq!("32000000", process(input));
    }
    #[test]
    fn test_process_2() {
        let input = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        assert_eq!("11687500", process(input));
    }
}
