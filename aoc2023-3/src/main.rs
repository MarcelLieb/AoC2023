use std::{fs, ops::Range, str::Chars};

use regex::Regex;

fn main() {
    let input: Vec<String> = fs::read_to_string("./input.txt")
        .unwrap()
        .to_ascii_lowercase()
        .split('\n')
        .map(str::to_string)
        .collect();
    let chars: Vec<_> = input
        .iter()
        .map(String::as_str)
        .map(str::chars)
        .map(Chars::collect::<Vec<_>>)
        .collect();

    let regex = Regex::new("[0-9]+").unwrap();

    /*
    */
    let part_one: u32 = input
        .iter()
        .map(|str| regex.find_iter(&str))
        .enumerate()
        .map(|(row, matches)| {
            matches
                .filter_map(|num| {
                    if check_boundary(&chars, row, num.start()..num.end()) {
                        Some(num.as_str().parse::<u32>().unwrap())
                    } else {
                        None
                    }
                })
                .sum::<u32>()
        })
        .sum();
    println!("{}", part_one);

    let part_two:u32 = chars.iter()
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(col, &c)| {
                    if c == '*' {
                        Some(find_gears(&input, row, col, regex.clone()))
                    }
                    else {
                        None
                    }
                })
                .sum::<u32>()
        })
        .sum();

    println!("{}", part_two);
}

fn check_boundary(input: &[Vec<char>], row: usize, pos: Range<usize>) -> bool {
    let rows = row.saturating_sub(1)..=(row + 1).min(input.len() - 1);

    for y in rows {
        let cols = pos.clone().next().unwrap().saturating_sub(1)..=(pos.clone().last().unwrap() + 1).min(input[y].len() - 1);
        for x in cols {
            if input[y][x].is_digit(10) || input[y][x] == '.' {
                continue;
            }
            return true;
        }
    }
    false
}

fn find_gears(input: &[String], row: usize, col: usize, regex: Regex) -> u32 {
    let mut matches: Vec<_> = regex.find_iter(&input[row]).collect();
    let rows = row.saturating_sub(1)..=(row + 1).min(input.len() - 1);
    for y in rows {
        if y != row {
            let mut mat = regex.find_iter(&input[y]).collect::<Vec<_>>();
            matches.append(&mut mat);
        }
    }
    let matches:Vec<_> = matches.iter()
        .filter(|mat| {
            let cols = (mat.start().saturating_sub(1))..(mat.end() + 1);
            cols.contains(&col)
        })
        .collect();
    if matches.len() != 2 {
        return 0;
    }
    matches[0].as_str().parse::<u32>().unwrap() * matches[1].as_str().parse::<u32>().unwrap()
}
