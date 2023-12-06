use std::fs;

fn main() {
    let input: Vec<_> = fs::read_to_string("./input.txt")
        .unwrap()
        .to_ascii_lowercase()
        .lines()
        .map(str::to_string)
        .collect();

    let part_one = parse_races(&input)
        .iter()
        .map(|&(time, distance)| (0..time, time, distance))
        .map(|(times, time, distance)| {
            times
                .map(move |speed| speed * (time.clone() - speed))
                .filter(move |&dist| dist > distance.clone())
                .count()
        })
        .reduce(|acc, pos| acc * pos)
        .unwrap();

    println!("{}", part_one);

    let (time, distance) = parse_long_race(&input);
    let part_two = (0..time)
        .map(|speed| speed * (time - speed))
        .filter(|&dist| dist > distance)
        .count();

    println!("{}", part_two);
}

fn parse_races(input: &[String]) -> Vec<(u64, u64)> {
    let times: Vec<_> = input[0]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    let races: Vec<_> = input[1]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    times
        .iter()
        .zip(races.iter())
        .map(|(&a, &b)| (a, b))
        .collect()
}

fn parse_long_race(input: &[String]) -> (u64, u64) {
    let time: u64 = input[0]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(str::to_string)
        .fold(String::new(), |acc, num| acc + &num)
        .parse()
        .unwrap();

    let distance: u64 = input[1]
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(str::to_string)
        .fold(String::new(), |acc, num| acc + &num)
        .parse()
        .unwrap();
    (time, distance)
}
