use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn part1(map: Vec<Vec<char>>, mut guard_position: (i32, i32)) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let moves = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

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

        if map[next.0][next.1] == '#' {
            current_move = (current_move + 1) % 4
        } else {
            guard_position = (next.0 as i32, next.1 as i32);
            visited.insert(next);
        }
    }

    visited.len()
}

fn part2(initial_map: Vec<Vec<char>>, initial_guard_position: (i32, i32)) -> usize {
    let mut res = 0;

    let map = initial_map.clone();
    let mut guard_position = initial_guard_position.clone();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let moves = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

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

        if map[next.0][next.1] == '#' {
            current_move = (current_move + 1) % 4
        } else {
            guard_position = (next.0 as i32, next.1 as i32);
            visited.insert(next);
        }
    }

    'outer: for v in visited {
        current_move = 0;
        let mut guard_position = initial_guard_position.clone();
        // try putting obstacle in visited
        let mut map_with_obstacle = map.clone();
        map_with_obstacle[v.0][v.1] = 'O';

        let mut total_visited: HashMap<(usize, usize), u32> = HashMap::new();

        loop {
            let next = (
                guard_position.0 + moves[current_move].0,
                guard_position.1 + moves[current_move].1,
            );

            for val in total_visited.values() {
                if val > &10 {
                    println!("detected loop");
                    res += 1;
                    continue 'outer;
                }
            }

            let next: (usize, usize) = {
                if let Ok(i) = usize::try_from(next.0) {
                    if let Ok(j) = usize::try_from(next.1) {
                        (i, j)
                    } else {
                        continue 'outer;
                    }
                } else {
                    continue 'outer;
                }
            };

            if next.0 >= map_with_obstacle.len() || next.1 >= map_with_obstacle[0].len() {
                continue 'outer;
            }

            if map_with_obstacle[next.0][next.1] == '#' || map_with_obstacle[next.0][next.1] == 'O'
            {
                current_move = (current_move + 1) % 4
            } else {
                guard_position = (next.0 as i32, next.1 as i32);
                *total_visited.entry(next).or_insert(1) += 1
            }
        }
    }

    println!("{res}");
    res
}

fn main() {
    let input = fs::read_to_string("../day06.input.txt").unwrap();
    let mut map: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        map.push(line.chars().collect::<Vec<char>>());
    }

    let mut guard_position: (i32, i32) = (0, 0);
    'outer: for (i, row) in map.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if char == &'^' {
                guard_position = (i as i32, j as i32);
                break 'outer;
            }
        }
    }

    part1(map.clone(), guard_position.clone());
    part2(map.clone(), guard_position.clone());
}
