use std::{
    collections::HashMap,
    fs,
};

fn main() {
    let input: String = fs::read_to_string("./input.txt")
        .unwrap()
        .to_ascii_lowercase()
        .to_string();
    let (seeds, maps) = parse_maps(&input);
    let part_one = seeds
        .iter()
        .map(|&seed| map_to_end(seed, &maps))
        .min()
        .unwrap();
    println!("{}", part_one);

    let part_two = seeds
        .iter()
        .zip(seeds.iter().skip(1))
        .step_by(2)
        .map(|(&start, &len)| (start..start + len))
        .flatten()  // 2_510_890_762 elements
        .map(|seed| map_to_end(seed, &maps))
        .min()
        .unwrap();
    println!("{}", part_two);
}

fn map_to_end(n: u64, maps: &[BigNumberMap]) -> u64 {
    let mut out = n;
    for map in maps.iter() {
        out = map.index(out);
    }
    out
}

fn parse_maps(input: &str) -> (Vec<u64>, Vec<BigNumberMap>) {
    let mut lines: Vec<_> = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect();
    lines.reverse();
    let seeds: Vec<_> = lines
        .pop()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();
    lines.pop();
    let mut maps: Vec<BigNumberMap> = Vec::new();
    let mut map = BigNumberMap::new();
    while !lines.is_empty() {
        let line = lines.pop().unwrap();
        if line.chars().next().unwrap().is_alphabetic() {
            maps.push(map);
            map = BigNumberMap::new();
            continue;
        }
        let range: Vec<_> = line
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect();
        map.insert_range(range[1], range[0], range[2])
    }
    maps.push(map);
    (seeds, maps)
}

struct BigNumberMap {
    offset_map: HashMap<(u64, u64), i64>,
}

impl BigNumberMap {
    fn new() -> Self {
        BigNumberMap {
            offset_map: HashMap::default(),
        }
    }

    fn insert_range(&mut self, from_start: u64, to_start: u64, length: u64) {
        let offset: i64 = to_start as i64 - from_start as i64;
        self.offset_map
            .insert((from_start, from_start + length - 1), offset);
    }

    fn index(&self, index: u64) -> u64 {
        for &(start, end) in self.offset_map.keys() {
            if index >= start && index <= end {
                return (self.offset_map[&(start, end)] + index as i64) as u64;
            }
        }
        return index;
    }
}
