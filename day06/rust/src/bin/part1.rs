use std::iter::zip;

fn get_total_travel(hold_time: &u32, millisecond: &u32, _distance: &u32) -> u32 {
    (millisecond - hold_time) * hold_time
}

fn process(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut ans = 1;

    let times = lines[0]
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .flat_map(|time| time.parse::<u32>().ok())
        .collect::<Vec<u32>>();
    let distances = lines[1]
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .flat_map(|distance| distance.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let times_and_distances = zip(&times, &distances).collect::<Vec<(&u32, &u32)>>();

    for item in times_and_distances {
        let mut win = 0;
        for hold_time in 0..=*item.0 {
            if get_total_travel(&hold_time, item.0, item.1) > *item.1 {
                // win
                win += 1;
            }
        }

        ans *= win;
    }

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

        assert_eq!(process(input), 288)
    }
}
