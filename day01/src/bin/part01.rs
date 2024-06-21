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

pub fn part_2() -> u32 {
    let buffer = fs::read_to_string("01-part1.txt").unwrap();

    let valid_input: HashMap<char, &str> = HashMap::from([
        ('1', "one"),
        ('2', "two"),
        ('3', "three"),
        ('4', "four"),
        ('5', "five"),
        ('6', "six"),
        ('7', "seven"),
        ('8', "eight"),
        ('9', "nine"),
    ]);

    let mut sum: u32 = 0;

    for line in buffer.lines() {
        let mut new_line = String::from(line);

        for (key_digit, value_string) in &valid_input {
            new_line = new_line.replace(
                value_string,
                // Satanic way to solve overlaping numbers, might change later (like twone)
                format!(
                    "{}{}{}",
                    value_string.chars().next().unwrap(),
                    key_digit,
                    value_string.chars().last().unwrap()
                )
                .as_str(),
            );
        }

        let digits = new_line
            .chars()
            .filter(|x| x.is_digit(10))
            .collect::<Vec<char>>();

        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        let number = format!("{}{}", first, last).parse::<u32>().unwrap();
        sum += number;
    }

    return sum;
}
