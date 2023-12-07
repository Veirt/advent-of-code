use std::cmp;
use std::collections::HashMap;

fn get_mapping(maps: &Vec<(u32, u32, u32)>, item: u32) -> u32 {
    for (destination_range_start, source_range_start, range_length) in maps {
        if item >= *source_range_start && item < source_range_start + range_length {
            let location = destination_range_start + (item - source_range_start);
            return location;
        }
    }

    item
}

fn get_location(maps: &HashMap<&str, Vec<(u32, u32, u32)>>, seed: u32) -> u32 {
    let soil = get_mapping(&maps["seed-to-soil"], seed);
    let fertilizer = get_mapping(&maps["soil-to-fertilizer"], soil);
    let water = get_mapping(&maps["fertilizer-to-water"], fertilizer);
    let light = get_mapping(&maps["water-to-light"], water);
    let temperature = get_mapping(&maps["light-to-temperature"], light);
    let humidity = get_mapping(&maps["temperature-to-humidity"], temperature);

    get_mapping(&maps["humidity-to-location"], humidity)
}

fn process(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();

    // (u32, u32, u32): destination_range_start, source_range_start, range_length
    let mut maps: HashMap<&str, Vec<(u32, u32, u32)>> = HashMap::new();

    let seeds = lines.first().unwrap().split(':').collect::<Vec<&str>>()[1]
        .trim()
        .split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .map(|seed| seed.parse::<u32>().expect("should be parsed correctly"))
        .collect::<Vec<u32>>();

    let seeds_range = seeds.chunks(2).collect::<Vec<_>>();

    let mut current_category = "";
    for line in &lines[2..] {
        let splitted = line.split(' ').collect::<Vec<&str>>();
        if splitted.len() == 2 {
            current_category = splitted.first().expect("should get the category")
        } else if splitted.len() == 3 {
            let numbers = splitted
                .iter()
                .map(|number| number.parse::<u32>().expect("should be parsed correctly"))
                .collect::<Vec<u32>>();

            maps.entry(current_category)
                .or_default()
                .push((numbers[0], numbers[1], numbers[2]));
        }
    }

    let mut min = 4294967295u32;

    for range in seeds_range {
        let start = range[0];
        let count = range[1];

        for seed in start..start + count {
            let location = get_location(&maps, seed);
            min = cmp::min(min, location);
        }
    }

    min
}

fn main() {
    let input = include_str!("../../../day05.input.txt");

    let result = process(input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

        assert_eq!(process(input), 46);
    }
}
