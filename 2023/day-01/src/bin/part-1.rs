fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

pub fn process(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut line_it = 
                line
                    .chars()
                    .filter_map(|c| c.to_digit(10));
            let first = line_it.next().unwrap_or(0);
            let last = line_it.last().unwrap_or(first);
            first * 10 + last
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input));
    }
}
