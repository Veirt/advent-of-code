use std::collections::HashMap;

fn get_location_maps(maps: &Vec<(u64, u64, u64)>, items: &Vec<u64>) -> HashMap<u64, u64> {
    let mut result: HashMap<u64, u64> = HashMap::new();

    for item in items {
        for (destination_range_start, source_range_start, range_length) in maps {
            if item >= source_range_start && item < &(source_range_start + range_length) {
                let location = destination_range_start + (item - source_range_start);
                *result.entry(*item).or_default() = location;
            }

            result.entry(*item).or_insert(*item);
        }
    }

    result
}

fn get_mapping(maps: &Vec<(u64, u64, u64)>, item: u64) -> u64 {
    for (destination_range_start, source_range_start, range_length) in maps {
        if item >= *source_range_start && item < source_range_start + range_length {
            let location = destination_range_start + (item - source_range_start);
            return location;
        }
    }

    item
}

fn get_location(maps: &HashMap<&str, Vec<(u64, u64, u64)>>, seed: u64) -> u64 {
    let soil = get_mapping(&maps["seed-to-soil"], seed);
    let fertilizer = get_mapping(&maps["soil-to-fertilizer"], soil);
    let water = get_mapping(&maps["fertilizer-to-water"], fertilizer);
    let light = get_mapping(&maps["water-to-light"], water);
    let temperature = get_mapping(&maps["light-to-temperature"], light);
    let humidity = get_mapping(&maps["temperature-to-humidity"], temperature);

    get_mapping(&maps["humidity-to-location"], humidity)
}

fn process(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<&str>>();

    // (u64, u64, u64): destination_range_start, source_range_start, range_length
    let mut maps: HashMap<&str, Vec<(u64, u64, u64)>> = HashMap::new();

    let seeds = lines.first().unwrap().split(':').collect::<Vec<&str>>()[1]
        .trim()
        .split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .map(|seed| seed.parse::<u64>().expect("should be parsed correctly"))
        .collect::<Vec<u64>>();

    let mut current_category = "";
    for line in &lines[2..] {
        let splitted = line.split(' ').collect::<Vec<&str>>();
        if splitted.len() == 2 {
            current_category = splitted.first().expect("should get the category")
        } else if splitted.len() == 3 {
            let numbers = splitted
                .iter()
                .map(|number| number.parse::<u64>().expect("should be parsed correctly"))
                .collect::<Vec<u64>>();

            maps.entry(current_category)
                .or_default()
                .push((numbers[0], numbers[1], numbers[2]));
        }
    }

    let seed_to_soil_maps: HashMap<u64, u64> = get_location_maps(&maps["seed-to-soil"], &seeds);
    let soil_to_fertilizer_maps: HashMap<u64, u64> = get_location_maps(
        &maps["soil-to-fertilizer"],
        &seed_to_soil_maps.values().cloned().collect::<Vec<u64>>(),
    );

    let fertilizer_to_water_maps: HashMap<u64, u64> = get_location_maps(
        &maps["fertilizer-to-water"],
        &soil_to_fertilizer_maps
            .values()
            .cloned()
            .collect::<Vec<u64>>(),
    );
    let water_to_light_maps: HashMap<u64, u64> = get_location_maps(
        &maps["water-to-light"],
        &fertilizer_to_water_maps
            .values()
            .cloned()
            .collect::<Vec<u64>>(),
    );
    let light_to_temperature_maps: HashMap<u64, u64> = get_location_maps(
        &maps["light-to-temperature"],
        &water_to_light_maps.values().cloned().collect::<Vec<u64>>(),
    );
    let temperature_to_humidity_maps: HashMap<u64, u64> = get_location_maps(
        &maps["temperature-to-humidity"],
        &light_to_temperature_maps
            .values()
            .cloned()
            .collect::<Vec<u64>>(),
    );
    let humidity_to_location_maps: HashMap<u64, u64> = get_location_maps(
        &maps["humidity-to-location"],
        &temperature_to_humidity_maps
            .values()
            .cloned()
            .collect::<Vec<u64>>(),
    );

    // dbg!(seed_to_soil_maps);
    // dbg!(soil_to_fertilizer_maps);
    // dbg!(fertilizer_to_water_maps);
    // dbg!(water_to_light_maps);
    // dbg!(light_to_temperature_maps);
    // dbg!(temperature_to_humidity_maps);
    // dbg!(humidity_to_location_maps);

    *humidity_to_location_maps.values().min().unwrap()
}

fn process2(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<&str>>();

    // (u64, u64, u64): destination_range_start, source_range_start, range_length
    let mut maps: HashMap<&str, Vec<(u64, u64, u64)>> = HashMap::new();

    let seeds = lines.first().unwrap().split(':').collect::<Vec<&str>>()[1]
        .trim()
        .split(' ')
        .collect::<Vec<&str>>()
        .iter()
        .map(|seed| seed.parse::<u64>().expect("should be parsed correctly"))
        .collect::<Vec<u64>>();

    let mut current_category = "";
    for line in &lines[2..] {
        let splitted = line.split(' ').collect::<Vec<&str>>();
        if splitted.len() == 2 {
            current_category = splitted.first().expect("should get the category")
        } else if splitted.len() == 3 {
            let numbers = splitted
                .iter()
                .map(|number| number.parse::<u64>().expect("should be parsed correctly"))
                .collect::<Vec<u64>>();

            maps.entry(current_category)
                .or_default()
                .push((numbers[0], numbers[1], numbers[2]));
        }
    }

    seeds
        .iter()
        .map(|seed| get_location(&maps, *seed))
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("../../../day05.input.txt");

    let result = process(input);
    let result2 = process2(input);
    println!("process(): {}", result);
    println!("process2(): {}", result2);
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

        assert_eq!(process(input), 35);
    }

    #[test]
    fn sample2() {
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

        assert_eq!(process2(input), 35);
    }
}
