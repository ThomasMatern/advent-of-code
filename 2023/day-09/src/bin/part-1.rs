#![allow(unused_variables,unused_imports,dead_code, unused_mut)]

use nom::{IResult, Parser, character::complete::{line_ending, space1, self}, multi::separated_list1};


fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

fn parse(i: &str) -> IResult<&str, Vec<Vec<isize>>> {
    let (i, histories) = 
        separated_list1(line_ending,
            separated_list1(space1, complete::i128.map(|x| x as isize))
        )
        .parse(i)?;
    Ok((i, histories))
}

pub fn process(i: &str) -> String {
    let (_, histories) = parse(i).unwrap();
    histories
        .iter()
        .map(|row| {
            let mut last_column:Vec<isize> = Vec::new();
            let mut row = row.clone();   
            loop {
                last_column.push(*row.last().unwrap());
                row = row.iter().zip(row.iter().skip(1))
                    .map(|(&a, &b)| b-a)
                    .collect::<Vec<_>>();
                if row.len() == 0 || row.iter().all(|&x| x == 0) {
                    break;
                }
            }
            last_column.iter().sum::<isize>()
            
        })
        .sum::<isize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
        assert_eq!("114", process(input));
    }
}
