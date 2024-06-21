use std::{collections::HashMap, fs};

fn main() {
    let buffer = fs::read_to_string("input1.txt").unwrap();
    let result = process(&buffer);
    dbg!(result);
}

fn process(input: &str) -> String {
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

    for line in input.lines() {
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

    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen",
        );

        assert_eq!(result, 281.to_string());
    }
}
