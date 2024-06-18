use std::fs;

pub fn part_1() -> u32 {
    let file = fs::read_to_string("03-part1.txt").unwrap();

    let symbols = Vec::from(['*', '$', '@', '=', '+', '-', '/', '%', '#', '&']);
    let mut all_numbers: Vec<String> = Vec::new();

    let lines: Vec<&str> = file.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        let mut num: Vec<(char, bool)> = Vec::new();

        let chars: Vec<char> = line.chars().collect();

        for (y, char) in chars.iter().enumerate() {
            if !char.is_numeric() {
                continue;
            }
            let next_element = chars.get(y + 1).unwrap_or(&'.');
            let top_element = lines
                .get(i + 1)
                .unwrap_or(&".")
                .chars()
                .nth(y)
                .unwrap_or('.');
            let mut bottom_element = '.';
            let mut diagonal_bottom_next = '.';
            if i != 0 {
                bottom_element = lines
                    .get(i - 1)
                    .unwrap_or(&".")
                    .chars()
                    .nth(y)
                    .unwrap_or('.');

                diagonal_bottom_next = lines
                    .get(i - 1)
                    .unwrap_or(&".")
                    .chars()
                    .nth(y + 1)
                    .unwrap_or('.');
            }
            let mut diagonal_bottom_last = '.';
            if i != 0 && y != 0 {
                diagonal_bottom_last = lines
                    .get(i - 1)
                    .unwrap_or(&".")
                    .chars()
                    .nth(y - 1)
                    .unwrap_or('.');
            }
            let diagonal_top_next = lines
                .get(i + 1)
                .unwrap_or(&".")
                .chars()
                .nth(y + 1)
                .unwrap_or('.');
            let mut diagonal_top_last = '.';
            let mut last_element = &'.';
            if y != 0 {
                last_element = chars.get(y - 1).unwrap_or(&'.');
                diagonal_top_last = lines
                    .get(i + 1)
                    .unwrap_or(&".")
                    .chars()
                    .nth(y - 1)
                    .unwrap_or('.');
            }

            let is_symbol = symbols.contains(&diagonal_top_next)
                || symbols.contains(&diagonal_top_last)
                || symbols.contains(&diagonal_bottom_next)
                || symbols.contains(&diagonal_bottom_last)
                || symbols.contains(&top_element)
                || symbols.contains(&bottom_element)
                || symbols.contains(&last_element)
                || symbols.contains(&next_element);

            if char.is_numeric() {
                num.push((*char, is_symbol));
            }

            if char.is_numeric() && !next_element.is_numeric() {
                let new_num: String = num.iter().map(|x| x.0).collect();
                if num.iter().any(|x| x.1) {
                    all_numbers.push(new_num);
                }
                num.clear();
            }
        }
    }

    return all_numbers.iter().map(|x| x.parse::<u32>().unwrap()).sum();
}
