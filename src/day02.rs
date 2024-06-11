pub fn part_1() -> u32 {
    let buffer = std::fs::read_to_string("02-part1.txt").unwrap();

    let red = 12;
    let blue = 14;
    let green = 13;

    let mut sum: u32 = 0;

    for line in buffer.lines() {
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

    return sum;
}

pub fn part_2() -> u32 {
    let buffer = std::fs::read_to_string("02-part1.txt").unwrap();

    let mut sum: u32 = 0;

    for line in buffer.lines() {
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

    return sum;
}
