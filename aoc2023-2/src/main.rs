use std::{collections::HashMap, fs};

fn main() {
    let input: Vec<String> = fs::read_to_string("./input.txt")
        .unwrap()
        .to_ascii_lowercase()
        .split('\n')
        .map(str::to_string)
        .collect();
    let balls = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let part_one: u32 = input
        .iter()
        .map(|line| parse_game(&line))
        .filter_map(|(game, counts)| {
            if counts["red"] <= balls["red"]
                && counts["green"] <= balls["green"]
                && counts["blue"] <= balls["blue"]
            {
                Some(game)
            } else {
                None
            }
        })
        .sum();

    println!("{}", part_one);

    let part_two: u32 = input
        .iter()
        .map(|line| parse_game(&line))
        .map(|(_, counts)| counts["green"] * counts["blue"] * counts["red"])
        .sum();

    println!("{}", part_two);
}

fn parse_game(game: &str) -> (u32, HashMap<&str, u32>) {
    let mut counter = HashMap::from([("red", 0), ("blue", 0), ("green", 0)]);
    let (prefix, data) = game.split_once(": ").unwrap();
    let game: u32 = prefix.split_whitespace().last().unwrap().parse().unwrap();
    let rounds: Vec<String> = data.split("; ").map(str::to_string).collect();
    for round in rounds.iter() {
        let colors: Vec<_> = round.split(", ").map(str::to_string).collect();
        for color in colors.iter() {
            let (count, color) = color.split_once(' ').unwrap();
            let count: u32 = count.parse().unwrap();
            let new_count = counter[color].max(count);
            *(counter.get_mut(color).unwrap()) = new_count;
        }
    }
    (game, counter)
}
