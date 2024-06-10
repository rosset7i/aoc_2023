use std::{collections::HashMap, fs};

pub fn part_1() -> u32 {
    let buffer = std::fs::read_to_string("01-part1.txt").unwrap();
    let valid_input = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut sum: u32 = 0;

    let mut digits: Vec<String> = Vec::new();
    for line in buffer.lines() {
        for digit in line.chars() {
            if valid_input.contains(&digit) {
                digits.push(digit.to_string());
            }
        }
        let number = digits.first().unwrap().to_string() + digits.last().unwrap();
        sum += number.parse::<u32>().unwrap();
        digits.clear();
    }
    return sum;
}

pub fn part_2() -> u32 {
    let buffer = fs::read_to_string("01-part1.txt").unwrap();
    let valid_input: HashMap<char, String> = HashMap::from([
        ('1', "one".to_string()),
        ('2', "two".to_string()),
        ('3', "three".to_string()),
        ('4', "four".to_string()),
        ('5', "five".to_string()),
        ('6', "six".to_string()),
        ('7', "seven".to_string()),
        ('8', "eight".to_string()),
        ('9', "nine".to_string()),
    ]);
    let mut sum: u32 = 0;

    let mut digits: Vec<String> = Vec::new();
    for line in buffer.lines() {
        let mut new_line = String::from(line);
        for input in &valid_input {
            new_line = new_line.replace(
                input.1,
                // Satanic way to solve overlaping numbers, might change later (like twone)
                format!(
                    "{}{}{}",
                    input.1.chars().rev().last().unwrap(),
                    input.0,
                    input.1.chars().last().unwrap()
                )
                .as_str(),
            )
        }
        for digit in new_line.chars() {
            if valid_input.contains_key(&digit) {
                digits.push(digit.to_string());
            }
        }

        let number = digits.first().unwrap().to_string() + digits.last().unwrap().as_str();
        sum += number.parse::<u32>().unwrap();
        digits.clear();
    }

    return sum;
}
