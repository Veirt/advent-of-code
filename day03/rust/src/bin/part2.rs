use std::{collections::HashMap, num::ParseIntError};

fn is_connected_to_gear(character: char) -> bool {
    character == '*' && !character.is_ascii_digit()
}

fn get_asterisk_locations(
    engine_schematic: &Vec<Vec<char>>,
    indices: &Vec<(usize, usize)>,
) -> Option<(usize, usize)> {
    let xy_len = engine_schematic.len() as i32;

    // check neighbor
    for (x, y) in indices {
        let x = *x as i32;
        let y = *y as i32;
        let start_x = x - 1;
        let start_y = y - 1;

        for x in start_x..=x + 1 {
            for y in start_y..=y + 1 {
                // filter supaya index lebih dari 0 semua
                // filter supaya index ga lebih dari panjang
                if x >= 0 && y >= 0 && x < xy_len && y < xy_len {
                    let x = x as usize;
                    let y = y as usize;
                    if is_connected_to_gear(engine_schematic[x][y]) {
                        return Some((x, y));
                    }
                }
            }
        }
    }

    None
}

fn process(input: &str) -> u64 {
    let mut full_engine_schematic: Vec<Vec<char>> = vec![];

    let mut gear_numbers: HashMap<(usize, usize), Vec<u64>> = HashMap::new();

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
                    let num_parsed: Result<u64, ParseIntError> = num.clone().parse();
                    if let Ok(parsed) = num_parsed {
                        let asterisk_locations =
                            get_asterisk_locations(&full_engine_schematic, &num_indices);

                        if let Some(asterisk_locations) = asterisk_locations {
                            gear_numbers
                                .entry(asterisk_locations)
                                .or_insert(vec![parsed as u64])
                                .push(parsed as u64);
                        }
                    };
                    num.clear();
                    num_indices.clear();
                }
            } else {
                let num_parsed: Result<u64, ParseIntError> = num.clone().parse();
                if let Ok(parsed) = num_parsed {
                    let asterisk_locations =
                        get_asterisk_locations(&full_engine_schematic, &num_indices);

                    if let Some(asterisk_locations) = asterisk_locations {
                        gear_numbers
                            .entry(asterisk_locations)
                            .or_insert(vec![parsed as u64])
                            .push(parsed as u64);
                    }
                };

                num.clear();
                num_indices.clear();
            }
        }
    }

    let mut gear_ratio = 0;
    for (_location, values) in gear_numbers.clone().into_iter() {
        if values.len() > 2 {
            let gear_power = values[1] * values[2];
            gear_ratio += gear_power;
        }
    }

    gear_ratio
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

        assert_eq!(process(input), 467835);
    }

    #[test]
    fn bigboy() {
        let input = include_str!("../../../day03.bigboy.txt");

        todo!("still doesn't work.");
        assert_eq!(process(input), 17158526595);
    }
}
