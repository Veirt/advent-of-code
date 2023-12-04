use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
};

fn is_connected_to_gear(character: char) -> bool {
    character == '*' && !character.is_ascii_digit()
}

fn get_asterisk_locations(
    engine_schematic: Vec<Vec<char>>,
    indices: Vec<(usize, usize)>,
) -> Option<(usize, usize)> {
    // let mut gear_locations: HashSet<(usize, usize)> = HashSet::new();
    let mut indices_to_check: HashSet<(i32, i32)> = HashSet::new();

    // check neighbor
    for (x, y) in indices.clone() {
        let x = x as i32;
        let y = y as i32;
        let start_x: i32 = x as i32 - 1;
        let start_y: i32 = y as i32 - 1;

        for x in start_x..=x + 1 {
            for y in start_y..=y + 1 {
                indices_to_check.insert((x, y));
            }
        }
    }

    // filter supaya index lebih dari 0 semua
    // filter supaya index ga lebih dari panjang
    let indices_to_check: HashSet<(usize, usize)> = indices_to_check
        .iter()
        .filter(|indices| indices.0 >= 0 && indices.1 >= 0)
        .filter(|indices| {
            let xy_len = engine_schematic.len() as i32;

            indices.0 < xy_len && indices.1 < xy_len
        })
        .map(|&(x, y)| (x as usize, y as usize))
        .collect();

    // hilangin indices_to_check sama indices yang di cek sekarang
    let indices_to_check = &indices_to_check - &indices.iter().cloned().collect();
    // println!("{:?}", indices_to_check);

    for indices in indices_to_check {
        let (x, y) = indices;
        if is_connected_to_gear(engine_schematic[x][y]) {
            return Some((x, y));
        }
    }

    None
}

fn process(input: &str) -> u32 {
    let mut full_engine_schematic: Vec<Vec<char>> = vec![];

    let mut gear_numbers: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

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
                    let num_parsed: Result<i32, ParseIntError> = num.clone().parse();
                    if let Ok(parsed) = num_parsed {
                        let asterisk_locations = get_asterisk_locations(
                            full_engine_schematic.clone(),
                            num_indices.clone(),
                        );

                        if asterisk_locations.is_some() {
                            let asterisk_locations = asterisk_locations.unwrap();

                            gear_numbers
                                .entry(asterisk_locations)
                                .or_insert(vec![parsed as u32])
                                .push(parsed as u32);
                        }
                    };
                    num.clear();
                    num_indices.clear();
                }
            } else {
                let num_parsed: Result<i32, ParseIntError> = num.clone().parse();
                if let Ok(parsed) = num_parsed {
                    let asterisk_locations =
                        get_asterisk_locations(full_engine_schematic.clone(), num_indices.clone());

                    if asterisk_locations.is_some() {
                        let asterisk_locations = asterisk_locations.unwrap();

                        gear_numbers
                            .entry(asterisk_locations)
                            .or_insert(vec![parsed as u32])
                            .push(parsed as u32);
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

    dbg!(&gear_numbers.clone());
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
}
