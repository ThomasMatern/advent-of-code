#![allow(unused_variables,unused_imports,dead_code, unused_mut)]

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

pub fn process(input: &str) -> String {
    input.chars()
        .enumerate()
        .scan(0, |floor, (idx, ch)| {
            match ch {
                '(' => *floor += 1,
                ')' => *floor -= 1,
            _ => unreachable!()
            };
            if *floor >= 0 {
                Some(idx+2)
            } else {
                None
            }        
        }).last().unwrap().to_string()

}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "()())(";
        assert_eq!("5", process(input));
    }
}
