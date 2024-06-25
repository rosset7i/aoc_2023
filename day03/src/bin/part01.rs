use std::{collections::HashSet, fs};

fn main() {
    let buffer = fs::read_to_string("input3.txt").unwrap();
    let result = process(&buffer);
    dbg!(result);
}

fn process(input: &str) -> String {
    let mut parts: Vec<Part> = Vec::new();

    let mut current_digit: Option<Part> = None;
    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                match current_digit {
                    None => current_digit = Some(Part::from(row as i16, col as i16, char)),
                    Some(ref mut current) => current.add_digit(char, row as i16, col as i16),
                }
            } else {
                if let Some(current_digit) = current_digit.take() {
                    parts.push(current_digit);
                }
            }
        }
    }

    dbg!(parts);
    return "".to_string();
}

#[derive(Debug)]
struct Part {
    value: String,
    points: HashSet<(i16, i16)>,
}

impl Part {
    fn from(row: i16, col: i16, ch: char) -> Self {
        let adjacent_points = HashSet::from([
            (row - 1, col - 1),
            (row - 1, col),
            (row - 1, col + 1),
            (row, col + 1),
            (row + 1, col + 1),
            (row + 1, col),
            (row + 1, col - 1),
            (row, col - 1),
        ]);

        return Self {
            value: String::from(ch),
            points: adjacent_points,
        };
    }

    fn add_digit(&mut self, digit: char, row: i16, col: i16) {
        self.value.push(digit);
        self.points.extend([
            (row - 1, col - 1),
            (row - 1, col),
            (row - 1, col + 1),
            (row, col + 1),
            (row + 1, col + 1),
            (row + 1, col),
            (row + 1, col - 1),
            (row, col - 1),
        ]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );

        assert_eq!(result, 4361.to_string());
    }
}
