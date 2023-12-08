#![allow(unused_variables,unused_imports,dead_code)]

fn main() {
    let input = include_str!("./input-2.txt");
    let output = process(input);
    dbg!(output);
}

pub fn process(_input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "";
        assert_eq!("", process(input));
    }
}
