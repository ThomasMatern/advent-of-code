fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

pub fn process(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let line = line
                .replace("one", "on1ne")
                .replace("two", "tw2wo")
                .replace("three", "thre3hree")
                .replace("four", "fou4our")
                .replace("five", "fiv5ive")
                .replace("six", "si6ix")
                .replace("seven", "seve7even")
                .replace("eight", "eigh8ight")
                .replace("nine", "nin9ine");
            let mut line_it = line
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
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input));
    }
}
