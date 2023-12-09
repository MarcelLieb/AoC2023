use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("./input.txt")
        .unwrap()
        .to_ascii_lowercase()
        .lines()
        .map(str::to_string)
        .collect();

    let numbers: Vec<_> = input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|str| str.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let part_one: i64 = numbers
        .iter()
        .map(|history| extrapolate_numbers_forward(&history))
        .sum();

    println!("{}", part_one);

    let part_two: i64 = numbers
        .iter()
        .map(|history| extrapolate_numbers_backwards(&history))
        .sum();

    println!("{}", part_two);
}

fn extrapolate_numbers_forward(numbers: &[i64]) -> i64 {
    let mut diffs: Vec<Vec<i64>> = Vec::new();
    diffs.push(numbers.to_vec());
    while !diffs.last().unwrap().iter().all(|&a| a == 0) {
        diffs.push(
            diffs
                .last()
                .unwrap()
                .windows(2)
                .map(|a| a[1] - a[0])
                .collect(),
        )
    }
    diffs
        .iter()
        .rev()
        .fold(0, |acc, diff| acc + diff.last().unwrap())
}

fn extrapolate_numbers_backwards(numbers: &[i64]) -> i64 {
    let mut diffs: Vec<Vec<i64>> = Vec::new();
    diffs.push(numbers.to_vec());
    while !diffs.last().unwrap().iter().all(|&a| a == 0) {
        diffs.push(
            diffs
                .last()
                .unwrap()
                .windows(2)
                .map(|a| a[1] - a[0])
                .collect(),
        )
    }
    diffs
        .iter()
        .rev()
        .fold(0, |acc, diff| diff.first().unwrap() - acc)
}
