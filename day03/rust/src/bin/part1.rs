use std::num::ParseIntError;

fn is_symbol(character: char) -> bool {
    character != '.' && !character.is_ascii_digit()
}

fn check_valid_number(engine_schematic: &Vec<Vec<char>>, indices: &Vec<(usize, usize)>) -> bool {
    let xy_len = engine_schematic.len() as i32;

    for (x, y) in indices {
        let x = *x as i32;
        let y = *y as i32;
        let start_x = x - 1;
        let start_y = y - 1;

        for x in start_x..=x + 1 {
            for y in start_y..=y + 1 {
                // filter supaya index lebih dari 0 semua
                // filter supaya index ga lebih dari panjang
                if x >= 0
                    && y >= 0
                    && x < xy_len
                    && y < xy_len
                    && is_symbol(engine_schematic[x as usize][y as usize])
                {
                    return true;
                }
            }
        }
    }

    false
}

fn process(input: &str) -> u32 {
    let mut full_engine_schematic: Vec<Vec<char>> = vec![];

    let mut sum = 0;

    let lines: Vec<&str> = input.lines().collect();

    for line in lines {
        full_engine_schematic.push(line.chars().collect());
    }

    for (i, schematic) in full_engine_schematic.iter().enumerate() {
        // println!("{:?}", schematic);
        let mut num = String::new();
        let mut num_indices: Vec<(usize, usize)> = vec![];

        for (j, character) in schematic.iter().enumerate() {
            if character.is_ascii_digit() {
                num.push(*character);
                num_indices.push((i, j));

                // dbg!(&num);
                // dbg!(j);
                if j == full_engine_schematic.len() - 1 {
                    let num_parsed: Result<i32, ParseIntError> = num.parse();
                    if let Ok(parsed) = num_parsed {
                        if check_valid_number(&full_engine_schematic, &num_indices) {
                            sum += parsed;
                        }
                    };

                    num.clear();
                    num_indices.clear();
                }
            } else {
                let num_parsed: Result<i32, ParseIntError> = num.parse();
                if let Ok(parsed) = num_parsed {
                    if check_valid_number(&full_engine_schematic, &num_indices) {
                        sum += parsed;
                    }
                };

                num.clear();
                num_indices.clear();
            }
        }
    }

    sum as u32
}

fn main() {
    let input = include_str!("../../../day03.input.txt");

    let result = process(input);

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "467..114..
...*......
..35...633
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(process(input), 4361);
    }

    #[test]
    fn bigboy() {
        let input = include_str!("../../../day03.bigboy.txt");

        assert_eq!(process(input), 258006204);
    }
}
