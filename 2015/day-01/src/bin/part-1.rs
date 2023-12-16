#![allow(unused_variables,unused_imports,dead_code, unused_mut)]


fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

pub fn process(input: &str) -> String {
    input.chars()
        .fold(0, |floor, ch| {
            match ch {
                '(' => floor + 1,
                ')' => floor - 1,
            _ => unreachable!()
            }
    }).to_string()
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
