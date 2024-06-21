fn main() {
    let buffer = std::fs::read_to_string("input1.txt").unwrap();
    let result = process(&buffer);
    dbg!(result);
}

fn process(input: &str) -> String {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let (_, cube_str) = line.split_once(':').unwrap();

        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        for set in cube_str.split(';') {
            for color in set.split(',') {
                let (count, color_name) = color
                    .trim()
                    .split_at(color.trim().find(|x: char| x.is_whitespace()).unwrap_or(0));

                let count: u32 = count.parse().unwrap();

                match color_name.trim() {
                    "red" => {
                        if count > red {
                            red = count;
                        }
                    }
                    "blue" => {
                        if count > blue {
                            blue = count;
                        }
                    }
                    "green" => {
                        if count > green {
                            green = count;
                        }
                    }
                    _ => (),
                }
            }
        }
        sum += red * green * blue;
    }

    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );

        assert_eq!(result, 2286.to_string());
    }
}
