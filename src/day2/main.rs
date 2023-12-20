use std::{collections::HashMap, i32};
#[path = "../utils.rs"]
mod utils;

const TEST_CASE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

fn main() {
    let (test_total_1, test_total_2) = parse_text(TEST_CASE);
    println!("Test Total1: {}", test_total_1);
    println!("Test Total2: {}", test_total_2);
    let text = utils::read_text_from_file("input/day2.txt");
    let (total_1, total_2) = parse_text(&text);
    println!("Total1: {}", total_1);
    println!("Total2: {}", total_2);
}

fn parse_text(text: &str) -> (i32, i32) {
    let mut total_1: i32 = 0;
    let mut total_2: i32 = 0;
    let mut i = 1;
    for line in text.lines() {
        let ball_str = line.split(":").collect::<Vec<&str>>()[1];
        let ball_str = ball_str.trim();
        let ball_str = ball_str.replace(";", ",");
        let max_map = get_max(&ball_str);

        let possible = is_possible(max_map.clone());
        if possible {
            total_1 += i;
        }

        // multiply the max values
        let mut max = 1;
        for (_, value) in max_map.iter() {
            max *= value;
        }
        total_2 += max;
        i += 1;
    }
    return (total_1, total_2);
}

fn get_max(line: &str) -> HashMap<&str, i32> {
    let mut max_map: HashMap<&str, i32> = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

    for ball in line.split(","){
        let ball = ball.trim();

        let number = ball.split(" ").collect::<Vec<&str>>()[0]
            .parse::<i32>()
            .unwrap();
        let color = ball.split(" ").collect::<Vec<&str>>()[1];
        let max = max_map.get(color);
        if max.is_some() {
            if number > *max.unwrap() {
                max_map.insert(color, number);
            }
        }
    }

    return max_map;
}

fn is_possible(max_map: HashMap<&str, i32>) -> bool {
    let spec_map: HashMap<&str, i32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    for (color, max) in max_map.iter() {
        let spec = spec_map.get(color);
        if spec.is_some() {
            if *max > *spec.unwrap() {
                return false;
            }
        }
    }

    return true;
}
