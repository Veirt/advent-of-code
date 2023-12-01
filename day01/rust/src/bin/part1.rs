fn main() {
    let input = include_str!("../../../day01.input.txt");

    let mut lines: Vec<&str> = input.split("\n").collect();
    lines.pop();

    let mut result = 0;

    for line in lines {
        let mut first_digit: Option<char> = None;
        let mut last_digit: Option<char> = None;

        for character in line.chars() {
            if character.is_digit(10) {
                first_digit = Some(character);
                break;
            }
        }

        for character in line.chars().rev() {
            if character.is_digit(10) {
                last_digit = Some(character);
                break;
            }
        }

        if first_digit.is_some() && last_digit.is_some() {
            let concatted_digit = format!("{}{}", first_digit.unwrap(), last_digit.unwrap());
            let num: u32 = concatted_digit.parse().unwrap();
            result += num;
        }
    }

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(part2(result), 280);
    }
