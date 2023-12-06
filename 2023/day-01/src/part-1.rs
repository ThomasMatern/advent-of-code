fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

pub fn process(input: &str) -> String
{
    let lines: Vec<&str> = input.split("\n").collect();
    let mut sum = 0;
    for line in lines {
    
        let mut first: i32 = -1;
        let mut last: i32 = -1;
        for  c in line.chars() {
            if c.is_digit(10) {
                let num = c.to_digit(10).unwrap_or(0) as i32;
                if first == -1 {
                    first = num;
                }
                last = num;
            }
        }
        sum += first * 10 + last;
    }
    sum.to_string()
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