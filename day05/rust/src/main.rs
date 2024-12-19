use regex::Regex;
use std::fs;

fn part1(page_orders: Vec<(u32, u32)>, updates: Vec<Vec<u32>>) -> u32 {
    let mut res = 0;

    'outer: for update in updates {
        let filtered_page_orders = page_orders
            .iter()
            .filter(|(x, y)| update.contains(x) && update.contains(y))
            .collect::<Vec<&(u32, u32)>>();

        // println!("{filtered_page_orders:?}");

        for (x, y) in filtered_page_orders {
            let x_idx = update.iter().position(|val| val == x).unwrap();
            let y_idx = update.iter().position(|val| val == y).unwrap();

            if x_idx > y_idx {
                continue 'outer;
            }
        }

        res += update[update.len() / 2]
    }

    println!("{res}");
    res
}

fn check_valid(page_orders: &Vec<&(u32, u32)>, update: &Vec<u32>) -> Option<(usize, usize)> {
    let filtered_page_orders = page_orders
        .iter()
        .filter(|(x, y)| update.contains(x) && update.contains(y))
        .collect::<Vec<&&(u32, u32)>>();

    for (x, y) in filtered_page_orders {
        let x_idx = update.iter().position(|val| val == x).unwrap();
        let y_idx = update.iter().position(|val| val == y).unwrap();

        if x_idx > y_idx {
            return Some((x_idx, y_idx));
        }
    }

    None
}

fn part2(page_orders: Vec<(u32, u32)>, updates: Vec<Vec<u32>>) -> u32 {
    let mut res = 0;

    for update in updates {
        let filtered_page_orders = page_orders
            .iter()
            .filter(|(x, y)| update.contains(x) && update.contains(y))
            .collect::<Vec<&(u32, u32)>>();

        let mut tmp_update = update.clone();

        // skip the valid ones
        if check_valid(&filtered_page_orders, &update).is_none() {
            continue;
        }

        loop {
            if let Some((x, y)) = check_valid(&filtered_page_orders, &tmp_update) {
                // correcting the invalid ones
                let x_idx = tmp_update
                    .iter()
                    .position(|val| val == &tmp_update[x])
                    .unwrap();
                let y_idx = tmp_update
                    .iter()
                    .position(|val| val == &tmp_update[y])
                    .unwrap();
                let tmp = tmp_update[x_idx];
                tmp_update[x_idx] = tmp_update[y_idx];
                tmp_update[y_idx] = tmp;
                // println!("{x_idx} {y_idx}");
                // println!("{update:?}");
                continue;
            }
            // println!("{tmp_update:?}");
            res += tmp_update[tmp_update.len() / 2];
            break;
        }
    }

    println!("{res}");
    res
}

fn main() {
    let input = fs::read_to_string("../day05.input.txt").unwrap();

    let mut page_orders: Vec<(u32, u32)> = vec![];
    let mut updates: Vec<Vec<u32>> = vec![];

    let re = Regex::new(r"\d+\|\d+").unwrap();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if re.is_match(line) {
            let splitted = line.split("|").collect::<Vec<&str>>();
            let x = splitted[0].parse::<u32>().unwrap();
            let y = splitted[1].parse::<u32>().unwrap();
            page_orders.push((x, y));
            continue;
        }

        updates.push(
            line.split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        );
    }

    part1(page_orders.clone(), updates.clone());
    part2(page_orders, updates);
}
