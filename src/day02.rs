pub fn part_1() -> u32 {
    let buffer = std::fs::read_to_string("02-part1.txt").unwrap();

    let red = 12;
    let blue = 14;
    let green = 13;
    let mut sum: u32 = 0;

    for line in buffer.lines() {
        let split = line.split(':').collect::<Vec<&str>>();

        let mut is_possible = true;

        let game = split
            .get(0)
            .unwrap()
            .replace("Game ", "")
            .parse::<u32>()
            .unwrap();

        let cube_sets = split.get(1).unwrap().split(';').collect::<Vec<&str>>();

        for set in cube_sets {
            let sets = set.split(',').collect::<Vec<&str>>();

            for color in sets {
                if color.contains("red") {
                    let count = color.replace("red", "").trim().parse::<u8>().unwrap();
                    if count > red {
                        is_possible = false;
                    }
                }
                if color.contains("blue") {
                    let count = color.replace("blue", "").trim().parse::<u8>().unwrap();
                    if count > blue {
                        is_possible = false;
                    }
                }
                if color.contains("green") {
                    let count = color.replace("green", "").trim().parse::<u8>().unwrap();
                    if count > green {
                        is_possible = false;
                    }
                }
            }
        }
        if is_possible {
            sum += game;
        }
    }

    return sum;
}
