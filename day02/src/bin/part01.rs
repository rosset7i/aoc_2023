fn main() {
    let buffer = std::fs::read_to_string("input1.txt").unwrap();
    let result = process(&buffer);
    dbg!(result);
}

fn process(input: &str) -> String {
    let red = 12;
    let blue = 14;
    let green = 13;

    let mut sum: u32 = 0;

    for line in input.lines() {
        let (game_str, cube_str) = line.split_once(':').unwrap();

        let mut is_possible = true;

        let game: u32 = game_str.replace("Game ", "").parse().unwrap();

        for set in cube_str.split(';') {
            for color in set.split(',') {
                let (count, color_name) = color
                    .trim()
                    .split_at(color.trim().find(|x: char| x.is_whitespace()).unwrap_or(0));

                let count: u32 = count.parse().unwrap();

                match color_name.trim() {
                    "red" => {
                        if count > red {
                            is_possible = false;
                        }
                    }
                    "blue" => {
                        if count > blue {
                            is_possible = false;
                        }
                    }
                    "green" => {
                        if count > green {
                            is_possible = false;
                        }
                    }
                    _ => (),
                }
            }
        }
        if is_possible {
            sum += game;
        }
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

        assert_eq!(result, 8.to_string());
    }
}
