fn process(input: &str) -> i32 {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut sum = 0;

    // non-mathy solution
    // god i'm stupid at math
    for line in lines {
        let sequence = line
            .split(' ')
            .map(|num| num.parse::<i32>().unwrap())
            .rev()
            .collect::<Vec<i32>>();

        // what the fuck part2 is actually only adding rev()

        let mut current: Vec<i32> = sequence.clone();

        let mut sequence_history: Vec<Vec<i32>> = vec![];

        loop {
            let mut temp: Vec<i32> = vec![];
            for i in 0..current.len() - 1 {
                temp.push(current[i + 1] - current[i]);
            }

            if current.iter().all(|num| *num == current[0]) {
                break;
            }

            current = temp;
            sequence_history.push(current.clone());
        }

        let mut to_add = 0;
        for sh in sequence_history.iter().rev().collect::<Vec<_>>() {
            to_add += *sh.last().unwrap();
            // dbg!(&to_add);
        }

        sum += sequence.last().unwrap() + to_add;
    }

    sum
}

fn main() {
    let input = include_str!("../../../day09.input.txt");

    let result = process(input);

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "10 13 16 21 30 45";

        assert_eq!(process(input), 5);
    }
}
