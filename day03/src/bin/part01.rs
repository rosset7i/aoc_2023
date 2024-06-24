use std::fs;

fn main() {
    let buffer = fs::read_to_string("input3.txt").unwrap();
    let result = process(&buffer);
    dbg!(result);
}

fn process(input: &str) -> String {
    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {}
    }
    return "".to_string();
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
