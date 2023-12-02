fn process(input: &str) -> u32 {
    let lines: Vec<&str> = input.split('\n').collect();

    let mut result = 0;

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let mut first_digit: Option<char> = None;
        let mut last_digit: Option<char> = None;

        for character in line.chars() {
            if character.is_ascii_digit() {
                first_digit = Some(character);
                break;
            }
        }

        for character in line.chars().rev() {
            if character.is_ascii_digit() {
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

    result
}
fn main() {
    let input = include_str!("../../../day01.input.txt");

    println!("{}", process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(process(input), 142);
    }

    #[test]
    fn bigboy() {
        let input = include_str!("../../../day01.bigboy.txt");

        assert_eq!(process(input), 55022487);
    }
}
