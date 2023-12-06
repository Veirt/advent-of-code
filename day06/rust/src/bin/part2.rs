fn get_total_travel(hold_time: &u64, millisecond: &u64, _distance: &u64) -> u64 {
    (millisecond - hold_time) * hold_time
}

fn process(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut ans = 1;

    let time = lines[0]
        .replace(' ', "")
        .replace("Time:", "")
        .parse::<u64>()
        .unwrap();

    let distance = lines[1]
        .replace(' ', "")
        .replace("Distance:", "")
        .parse::<u64>()
        .unwrap();

    let mut win = 0;
    for hold_time in 0..=time {
        // win
        if get_total_travel(&hold_time, &time, &distance) > distance {
            win += 1;
        }
    }

    ans *= win;

    ans
}

fn main() {
    let input = include_str!("../../../day06.input.txt");
    let result = process(input);

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(process(input), 71503)
    }
}
