use std::{collections::HashMap, fs};

fn main() {
    let input: Vec<String> = fs::read_to_string("./input.txt")
        .unwrap()
        .to_ascii_lowercase()
        .split('\n')
        .map(str::to_string)
        .collect();

    let part_one: u32 = input
        .iter()
        .map(|line| parse_card(line))
        .filter_map(|(_, winning, ticket)| {
            ticket
                .iter()
                .filter(|&i| winning.contains(i))
                .enumerate()
                .map(|(i, _)| i)
                .max()
        })
        .map(|i| 2_u32.pow(i as u32))
        .sum();

    println!("{}", part_one);

    let winnings: Vec<_> = input
        .iter()
        .map(|line| parse_card(line))
        .map(|(number, winning, ticket)| {
            ticket
                .iter()
                .filter(|&i| winning.contains(i))
                .enumerate()
                .map(|(i, _)| i + number as usize)
                .collect::<Vec<_>>()
        })
        .collect();

    let mut counter: HashMap<usize, u64> = HashMap::new();

    for (i, wins) in winnings.iter().enumerate() {
        counter.entry(i).or_insert(1);
        let multiplier = counter[&i];
        for &number in wins.iter() {
            let count = counter.entry(number).or_insert(1);
            *count = *count + multiplier;
        }
    }
    let part_two: u64 = counter.values().sum();

    println!("{}", part_two);
}

fn parse_card(card: &str) -> (u32, Vec<u32>, Vec<u32>) {
    let (card, numbers) = card.split_once(": ").unwrap();
    let mut card = card.split_whitespace();
    card.next();
    let card: u32 = card.next().unwrap().parse().unwrap();

    let (winning, ticket) = numbers.split_once("| ").unwrap();

    let winning: Vec<_> = winning
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    let numbers: Vec<u32> = ticket
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    (card, winning, numbers)
}
