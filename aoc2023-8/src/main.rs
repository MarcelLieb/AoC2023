use std::{collections::HashMap, fs};

use num::Integer;

fn main() {
    let input: Vec<String> = fs::read_to_string("./input.txt")
        .unwrap()
        .to_ascii_lowercase()
        .lines()
        .map(str::to_string)
        .collect();
    let (lr, map) = parse_map(&input);

    let place = "aaa";
    let instructions = lr.iter().map(usize::to_owned).cycle();

    let part_one = steps_to_end(place, instructions.clone(), &map, false);

    println!("{}", part_one);

    let part_two: u64 = map
        .keys()
        .filter(|&&d| d.chars().nth(2).unwrap() == 'a')
        .map(|&start| steps_to_end(start, instructions.clone(), &map, true))
        .fold(1, |acc: u64, x| acc.lcm(&x));

    println!("{}", part_two);
}

fn steps_to_end(
    start: &str,
    instructions: impl Iterator<Item = usize>,
    map: &HashMap<&str, [String; 2]>,
    part_two: bool,
) -> u64 {
    let mut steps = 0;
    let mut place = start;
    let mut instructions = instructions;
    while !part_two && place != "zzz" || part_two && !place.ends_with('z') {
        let direction = instructions.next().unwrap();
        place = &map[place][direction];
        steps += 1;
    }
    steps
}

fn parse_map(input: &[String]) -> (Vec<usize>, HashMap<&str, [String; 2]>) {
    let lr: Vec<_> = input[0]
        .chars()
        .map(|c| if c == 'l' { 0 } else { 1 })
        .collect();

    let mut map = HashMap::new();
    input[2..]
        .iter()
        .map(|line| {
            let (from, to) = line.split_once(" = ").unwrap();
            let (left, right) = to.split_once(", ").unwrap().to_owned();
            let mut left = left.to_string();
            let mut right = right.to_string();
            left.remove(0);
            right.remove(right.len() - 1);
            (from, [left, right])
        })
        .for_each(|(from, to)| {
            map.insert(from, to);
        });
    (lr, map)
}
