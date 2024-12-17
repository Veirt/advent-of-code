use std::fs;

fn part1(letters: Vec<Vec<&str>>) -> u32 {
    let mut res = 0;

    for (i, row) in letters.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if *char == "X" {
                // Checking front
                if j + 3 < row.len() {
                    let front = &letters[i][j..j + 4];

                    // println!("{front:?}");
                    if front.join("") == "XMAS" {
                        res += 1;
                    }
                }

                // Checking back
                if j.checked_sub(3).is_some() {
                    let back = &letters[i][j - 3..=j];

                    if back.join("") == "SAMX" {
                        res += 1
                    }
                }

                // Checking top
                if i > 2 {
                    let mut top = String::new();
                    for k in 0..=3 {
                        top += letters[i - k][j]
                    }

                    // println!("{top}")
                    if top == "XMAS" {
                        res += 1
                    }
                }

                // Checking bottom
                if i < letters.len() - 3 {
                    let mut bottom = String::new();
                    for k in 0..=3 {
                        bottom += letters[i + k][j]
                    }

                    if bottom == "XMAS" {
                        res += 1
                    }
                }

                // Checking diagonal down-right
                if i < letters.len() - 3 && j + 3 < row.len() {
                    let mut diagonal_down_right = String::new();
                    for k in 0..=3 {
                        diagonal_down_right += letters[i + k][j + k];
                    }

                    if diagonal_down_right == "XMAS" {
                        res += 1
                    }
                }

                // Checking diagonal down-left
                if j.checked_sub(3).is_some() && i < letters.len() - 3 {
                    let mut diagonal_down_left = String::new();

                    for k in 0..=3 {
                        diagonal_down_left += letters[i + k][j - k];
                    }

                    if diagonal_down_left == "XMAS" {
                        res += 1
                    }
                }

                // Checking diagonal up-right
                if i > 2 && j + 3 < row.len() {
                    let mut diagonal_up_right = String::new();

                    for k in 0..=3 {
                        diagonal_up_right += letters[i - k][j + k];
                    }

                    // println!("{diagonal_up_right}");
                    if diagonal_up_right == "XMAS" {
                        res += 1
                    }
                }

                // Checking diagonal up-left
                if j.checked_sub(3).is_some() && i > 2 {
                    let mut diagonal_up_left = String::new();

                    for k in 0..=3 {
                        diagonal_up_left += letters[i - k][j - k];
                    }

                    // println!("{diagonal_up_left} {i} {j}");
                    if diagonal_up_left == "XMAS" {
                        res += 1;
                    }
                }

                // println!("{i} {j} {char}");
            }
        }
    }

    println!("{res}");

    res
}

fn part2(letters: Vec<Vec<&str>>) -> u32 {
    let mut res = 0;

    for (i, row) in letters.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if *char == "A" && i > 0 && j > 0 && i < letters.len() - 1 && j < row.len() - 1 {
                let mut left = String::new();
                left += letters[i - 1][j - 1];
                left += "A";
                left += letters[i + 1][j + 1];

                let mut right = String::new();
                right += letters[i - 1][j + 1];
                right += "A";
                right += letters[i + 1][j - 1];

                if (left == "MAS" || left == "SAM") && (right == "MAS" || right == "SAM") {
                    // println!("{left} {right}");
                    res += 1
                }
            }
        }
    }

    println!("{res}");
    res
}

fn main() {
    let input = fs::read_to_string("../day04.input.txt").unwrap();
    let mut letters: Vec<Vec<&str>> = vec![];
    for line in input.split("\n") {
        if !line.is_empty() {
            letters.push(line.split("").filter(|letter| !letter.is_empty()).collect());
        }
    }

    part1(letters.clone());
    part2(letters.clone());
}
