use std::{collections::HashMap, fs};

use regex::Regex;

fn main() {
    let input = fs::read_to_string("./input.txt")
        .unwrap()
        .to_ascii_lowercase();
    let regex_one = Regex::new("[1-9]").unwrap();
    let regex_two = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[1-9]").unwrap();
    let map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    let part_one: u32 = input
        .to_ascii_lowercase()
        .split('\n')
        .map(|line| find_all_matches(line, regex_one.clone()))
        .map(|line| line.iter().map(|&s| map[s]).collect::<Vec<u32>>())
        .map(|line| line[0] * 10 + line.last().unwrap())
        .sum();
    println!("Part One {part_one}");
    let part_two: u32 = input
        .to_ascii_lowercase()
        .split('\n')
        .map(|line| find_all_matches(line, regex_two.clone()))
        .map(|line| line.iter().map(|&s| map[s]).collect::<Vec<u32>>())
        .map(|line| line[0] * 10 + line.last().unwrap())
        .sum();
    println!("Part two: {part_two}");
}

fn find_all_matches(input: &str, regex: Regex) -> Vec<&str> {
    let mut matches = Vec::new();
    let mut i = 0;
    while i < input.len() {
        let start = regex.find_at(input, i);
        if start.is_none() {
            break;
        }
        matches.push(start.unwrap().as_str());
        i = start.unwrap().start() + 1;
    }
    matches
}
