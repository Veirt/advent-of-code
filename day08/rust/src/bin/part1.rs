use std::collections::HashMap;

const LEFT: usize = 0;
const RIGHT: usize = 1;

fn process(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut networks: HashMap<String, (String, String)> = HashMap::new();

    let instructions = lines[0];

    for line in &lines[2..] {
        let line = line.replace(' ', "");
        let splitted: Vec<&str> = line.split('=').collect();

        let key = splitted[0];

        let next = splitted[1].replace(&['(', ')'], "");
        let next = next.split(',').collect::<Vec<&str>>();

        let next_left = next[LEFT].to_string();
        let next_right = next[RIGHT].to_string();

        networks.insert(key.to_string(), (next_left, next_right));
    }

    let mut current = "AAA";
    let mut steps = 0;
    loop {
        for instruction in instructions.chars() {
            if instruction == 'L' {
                current = &networks.get(current).unwrap().0;
            } else {
                current = &networks.get(current).unwrap().1;
            }

            steps += 1;
        }

        if current == "ZZZ" {
            break;
        }
    }

    steps
}

fn main() {
    let input = include_str!("../../../day08.input.txt");

    let result = process(input);

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input1 = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

        assert_eq!(process(input1), 2);

        let input2 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

        assert_eq!(process(input2), 6);
    }
}
