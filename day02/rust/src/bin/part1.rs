use std::collections::HashMap;
use std::collections::HashSet;

fn process(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();

    let cube_limit = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut all_games: HashSet<u64> = HashSet::new();
    let mut invalid_games: HashSet<u64> = HashSet::new();

    for line in lines {
        let splitted_input: Vec<&str> = line.split(": ").collect();
        let game_number = splitted_input[0]
            .strip_prefix("Game ")
            .expect("should stripped correctly")
            .parse::<u64>()
            .expect("should be a valid number");
        let game_info = splitted_input[1];

        let game_info_splitted: Vec<&str> = game_info.split("; ").collect();
        'outer: for game_info in game_info_splitted {
            let cube_rolls: Vec<&str> = game_info.split(", ").collect();
            for cube_roll in cube_rolls {
                let count_and_color: Vec<&str> = cube_roll.split(' ').collect();
                let count: u32 = count_and_color[0]
                    .parse()
                    .expect("should be a valid number");
                let color = count_and_color[1];

                let count_limit = cube_limit.get(color).expect("color key should be found");

                if &count > count_limit {
                    // not valid
                    invalid_games.insert(game_number);
                    break 'outer;
                }
            }
        }
        all_games.insert(game_number);
    }

    let result: u64 = (&all_games - &invalid_games).iter().sum();
    result
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

        assert_eq!(process(input), 8);
    }
}
