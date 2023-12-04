use std::hash::Hash;
use std::{
    collections::HashMap,
    fs,
    ops::{Deref, DerefMut, Index},
};

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

    let mut counter: DefaultHashMap<usize, u32> = DefaultHashMap::new(1_u32);

    for (i, wins) in winnings.iter().enumerate() {
        let multiplier = *(counter.get_default(i));
        for &number in wins.iter() {
            *counter.entry_default(number) += multiplier;
        }
    }
    let part_two: u32 = counter.values().sum();

    println!("{}", part_two);
}

fn parse_card(card: &str) -> (u32, Vec<u32>, Vec<u32>) {
    let (card, numbers) = card.split_once(": ").unwrap();
    let card: u32 = card.split_whitespace().nth(1).unwrap().parse().unwrap();

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

struct DefaultHashMap<K: Eq + PartialEq + Hash, V: Clone> {
    inner: HashMap<K, V>,
    default_value: V,
}

impl<K: Eq + PartialEq + Hash, V: Clone> DefaultHashMap<K, V> {
    fn new(default: V) -> DefaultHashMap<K, V> {
        DefaultHashMap {
            inner: HashMap::new(),
            default_value: default,
        }
    }

    fn entry_default(&mut self, key: K) -> &mut V {
        let default = self.default_value.clone();
        self.entry(key).or_insert(default)
    }

    fn get_default(&mut self, key: K) -> &V {
        let default = self.default_value.clone();
        self.entry(key).or_insert(default)
    }
}

impl<K: Eq + PartialEq + Hash, V: Clone> Index<K> for DefaultHashMap<K, V> {
    type Output = V;
    /*
    Don't use this if you want the entry to be added
     */
    fn index(&self, index: K) -> &Self::Output {
        match self.inner.get(&index) {
            Some(value) => value,
            None => &self.default_value,
        }
    }
}

impl<K: Eq + PartialEq + Hash, V: Clone> Deref for DefaultHashMap<K, V> {
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<K: Eq + PartialEq + Hash, V: Clone> DerefMut for DefaultHashMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
