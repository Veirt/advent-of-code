use std::fs;

use regex::Regex;

fn part1(hay: &str) -> u32 {
    let mut res = 0;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    println!("{}", hay);
    for (_, [x, y]) in re.captures_iter(&hay).map(|c| c.extract()) {
        let x = x.parse::<u32>().unwrap();
        let y = y.parse::<u32>().unwrap();
        res += x * y
    }

    println!("{}", res);
    res
}

fn part2(hay: &str) -> u32 {
    let s2 = Regex::new(r#"don\'t\(\).*?do\(\)"#)
        .unwrap()
        .replace_all(&hay, "")
        .to_string();

    part1(&s2)
}

fn main() {
    let hay = fs::read_to_string("../day03.input.txt").unwrap();
    part1(&hay);
    part2(&hay);
}
