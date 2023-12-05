use std::collections::{HashMap, HashSet};

fn process(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    let mut scratchcards: HashMap<usize, u32> = HashMap::new();

    for line in lines {
        let game_number = line
            .split(':')
            .next()
            .expect("should found the game number")
            .replace("Card ", "")
            .trim()
            .parse::<usize>()
            .expect("should parsed correctly");

        scratchcards.entry(game_number).or_insert(1);

        let split_numbers: Vec<&str> = line.split('|').collect();

        let winning_numbers = split_numbers[0]
            .split(':')
            .last()
            .expect("should get the winning numbers");
        let winning_numbers: HashSet<u32> = winning_numbers
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .filter_map(|number| number.parse::<u32>().ok())
            .collect();

        let my_numbers: HashSet<u32> = split_numbers[1]
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .filter_map(|number| number.parse::<u32>().ok())
            .collect();

        let matched_numbers: Vec<&u32> = winning_numbers.intersection(&my_numbers).collect();
        let matched_numbers_len = matched_numbers.len();

        let scratchcards_got: Vec<usize> =
            (game_number + 1..=game_number + matched_numbers_len).collect();

        let scratchcard_count = *scratchcards
            .get(&game_number)
            .expect("should get the count");

        for _ in 0..scratchcard_count {
            for scratchcard in scratchcards_got.clone() {
                *scratchcards.entry(scratchcard).or_insert(1) += 1
            }
        }
    }

    scratchcards.values().sum()
}

fn main() {
    let input = include_str!("../../../day04.input.txt");

    let result = process(input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(process(input), 30);
    }

    #[test]
    fn bigboy() {
        let input = include_str!("../../../day04.bigboy.txt");

        assert_eq!(process(input), 211552);
    }
}
