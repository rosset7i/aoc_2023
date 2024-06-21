use std::{collections::HashMap, fs};

fn main() {
    let buffer = fs::read_to_string("input1.txt").unwrap();
    let result = process(&buffer);
    dbg!(result);
}

fn process(input: &str) -> String {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let digits = line
            .chars()
            .filter(|x| x.is_digit(10))
            .collect::<Vec<char>>();

        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        let number = format!("{}{}", first, last).parse::<u32>().unwrap();
        sum += number;
    }

    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet",
        );

        assert_eq!(result, 142.to_string());
    }
}
