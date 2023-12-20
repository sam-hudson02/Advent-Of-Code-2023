use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

const SPELLED_NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let text = read_text_from_file("input/day1.txt");
    let total = parse_text(&text);
    println!("Total: {}", total);
}

fn read_text_from_file(path: &str) -> String {
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    return contents;
}

fn parse_text(text: &str) -> i32 {
    let mut total: i32 = 0;
    for line in text.lines() {
        let number: i32 = get_number(line);
        total += number;
    }
    return total;
}

fn get_number(line: &str) -> i32 {
    let line_length: i32 = line.len() as i32;
    let mut i: i32 = 0;

    let mut number_string: String = String::new();

    let mut seen_str: String = String::new();

    // find the first digit
    loop {
        let c: char = line.chars().nth(i as usize).unwrap();
        seen_str.push(c);
        if c.is_digit(10) {
            number_string.push(c);
            break;
        }

        let num_in_str = number_in_string(&seen_str);
        if num_in_str > 0 {
            number_string.push_str(&num_in_str.to_string());
            break;
        }
        i += 1;
    }
    seen_str.clear();

    // find the last digit
    i = line_length - 1;
    loop {
        let c: char = line.chars().nth(i as usize).unwrap();
        // push char to the front of seen_str
        seen_str.insert(0, c);

        if c.is_digit(10) {
            number_string.push(c);
            break;
        }

        let num_in_str = number_in_string(&seen_str);
        if num_in_str > 0 {
            number_string.push_str(&num_in_str.to_string());
            break;
        }

        i -= 1;
    }

    let number = number_string.parse::<i32>().unwrap();

    println!("{} -> {}", line, number);
    return number;
}

fn number_in_string(seen_str: &str) -> i32 {
    let spell_map: HashMap<&str, i32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    for number in SPELLED_NUMBERS.iter() {
        if seen_str.contains(number) {
            return *spell_map.get(*number).unwrap();
        }
    }
    return 0;
}
