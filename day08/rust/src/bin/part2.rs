use std::collections::HashMap;

use num::Integer;

const LEFT: usize = 0;
const RIGHT: usize = 1;

fn process(input: &str) -> u64 {
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

    let mut currents = networks
        .keys()
        .filter(|key| key.ends_with('A'))
        .collect::<Vec<_>>();

    let mut intersect_z = vec![0; currents.len()];

    let mut steps = 0;
    'outer: loop {
        for instruction in instructions.chars() {
            for (i, current) in currents.clone().iter().enumerate() {
                if instruction == 'L' {
                    currents[i] = &networks.get(*current).unwrap().0;
                } else {
                    currents[i] = &networks.get(*current).unwrap().1;
                }
            }

            steps += 1;
            for (i, curr) in currents.iter().enumerate() {
                if curr.ends_with('Z') {
                    intersect_z[i] = steps;
                    let end_with_z_steps =
                        intersect_z.iter().filter(|v| **v != 0).collect::<Vec<_>>();
                    if end_with_z_steps.len() == currents.len() {
                        break 'outer;
                    }
                }
            }
        }
    }

    let end_with_z_steps = intersect_z.iter().filter(|v| **v != 0).collect::<Vec<_>>();
    let mut ans: u64 = *end_with_z_steps[0];
    // dbg!(ans);

    for i in &end_with_z_steps[1..] {
        // dbg!(i);
        ans = ans.lcm(*i);
    }

    ans
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
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

        assert_eq!(process(input), 6);
    }
}
