use std::collections::HashSet;

fn process(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    let mut total_point: u32 = 0;

    for line in lines {
        let _game_number = line
            .split(':')
            .next()
            .expect("should found the game number");

        let split_numbers: Vec<&str> = line.split('|').collect();
        let winning_numbers = split_numbers[0]
            .split(':')
            .last()
            .expect("should get the winning numbers");

        let winning_numbers: Vec<&str> = winning_numbers.split(' ').collect();

        // parse to u32
        let winning_numbers: HashSet<u32> = winning_numbers
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

        let mut current_point = 0;
        let exp: i32 = (matched_numbers.len() as i32) - 1;
        if exp >= 0 {
            current_point = 2i32.pow(exp as u32);
        }

        total_point += current_point as u32;
    }

    total_point
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

        assert_eq!(process(input), 13);
    }

    #[test]
    fn bigboy() {
        let input = include_str!("../../../day04.bigboy.txt");

        assert_eq!(process(input), 1475828);
    }
}
