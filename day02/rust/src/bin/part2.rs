use std::collections::HashMap;

fn process(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;

    for line in lines {
        let mut cube_minimum: HashMap<&str, u32> = HashMap::new();

        let splitted_input: Vec<&str> = line.split(": ").collect();
        let _game_number = splitted_input[0]
            .strip_prefix("Game ")
            .expect("should stripped correctly")
            .parse::<u32>()
            .expect("should be a valid number");
        let game_info = splitted_input[1];

        let game_info_splitted: Vec<&str> = game_info.split("; ").collect();
        for game_info in game_info_splitted {
            let cube_rolls: Vec<&str> = game_info.split(", ").collect();
            for cube_roll in cube_rolls {
                let count_and_color: Vec<&str> = cube_roll.split(' ').collect();
                let count: u32 = count_and_color[0]
                    .parse()
                    .expect("should be a valid number");
                let color = count_and_color[1];

                let opt_color_min = cube_minimum.get(color);
                if let Some(color_min) = opt_color_min {
                    if &count > color_min {
                        *cube_minimum.get_mut(color).unwrap() = count;
                    }
                } else {
                    cube_minimum.insert(color, count);
                }
            }
        }

        let mut current_game_power = 1;
        for (_key, value) in cube_minimum.clone().into_iter() {
            current_game_power *= value;
        }

        sum += current_game_power;
    }

    sum
}

fn main() {
    let input = include_str!("../../../day02.input.txt");

    let result = process(input);

    println!("{}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(process(input), 2286);
    }

    #[test]
    fn bigboy() {
        todo!("fix this. still wrong.");

        let input = include_str!("../../../day02.bigboy.txt");

        assert_eq!(process(input), 15913360);
    }
}
