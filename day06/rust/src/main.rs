use std::{collections::HashSet, fs};

fn part1(map: Vec<Vec<&str>>, mut guard_position: (i32, i32)) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let moves = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    println!("{guard_position:?}");

    let mut current_move = 0;
    'outer: loop {
        let next = (
            guard_position.0 + moves[current_move].0,
            guard_position.1 + moves[current_move].1,
        );

        let next: (usize, usize) = {
            if let Ok(i) = usize::try_from(next.0) {
                if let Ok(j) = usize::try_from(next.1) {
                    (i, j)
                } else {
                    panic!("Should not happen");
                }
            } else {
                panic!("Should not happen");
            }
        };

        if next.0 >= map.len() || next.1 >= map[0].len() {
            break 'outer;
        }

        // println!("{next:?}");
        if map[next.0][next.1] == "#" {
            current_move = (current_move + 1) % 4
        } else {
            guard_position = (next.0 as i32, next.1 as i32);
            visited.insert(next);
        }
    }

    println!("{}", visited.len());
    visited.len()
}

fn main() {
    let input = fs::read_to_string("../day06.example.txt").unwrap();
    let mut map: Vec<Vec<&str>> = vec![];
    for line in input.lines() {
        map.push(line.split("").collect::<Vec<&str>>());
    }

    let mut guard_position: (i32, i32) = (0, 0);
    'outer: for (i, row) in map.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if char == &"^" {
                guard_position = (i as i32, j as i32);
                break 'outer;
            }
        }
    }

    part1(map.clone(), guard_position.clone());
}
