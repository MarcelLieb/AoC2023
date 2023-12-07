use std::fs;

static ORDER: &str = "23456789TJQKA";
static ORDER_2: &str = "J23456789TQKA";



fn main() {
    let input: Vec<String> = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect();

    let mut game: Vec<_> = input
        .iter()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(hand, bid)| (Hand::new(hand, false), bid.parse::<u32>().unwrap()))
        .collect();

    game.sort();

    let part_one: u32 = game.iter()
        .enumerate()
        // .inspect(|(place, (hand, bid))| println!("Place:{} \t Hand:{}\t Value:{} \t Bid:{}", place + 1, hand.cards, hand.value, bid))
        .map(|(place, (_, bid))| (place + 1) as u32 * bid)
        .sum();

    println!("{}", part_one);

    let mut game_joker: Vec<_> = input
        .iter()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(hand, bid)| (Hand::new(hand, true), bid.parse::<u32>().unwrap()))
        .collect();

    game_joker.sort();

    let part_two: u32 = game_joker.iter()
        .enumerate()
        //.inspect(|(place, (hand, bid))| println!("Place:{} \t Hand:{}\t Value:{} \t Bid:{}", place + 1, hand.sorted, hand.value, bid))
        .map(|(place, (_, bid))| (place + 1) as u32 * bid)
        .sum();

    println!("{}", part_two);
}


#[derive(PartialEq, Eq, Clone, Debug)]
struct Hand {
    cards: String,
    value: u8,
    joker: bool,
    sorted: String,
}

impl Hand {
    fn new(cards: &str, joker: bool) -> Self {
        let mut ordered: Vec<_> = cards.chars().map(|c| (ORDER.find(c).unwrap(), c)).collect();
        ordered.sort_by_key(|a| a.0);
        let sorted = ordered.clone().iter().map(|(_, c)| c).collect();
        ordered.dedup();
        let mut value = match ordered.len() {
            1 => 6,
            2 => {
                let occurrences = cards.chars().filter(|&c| c == ordered[0].1).count();
                if occurrences == 1 || occurrences == 4 {
                    5
                } else {
                    4
                }
            }
            3 => {
                let mut occurrences = 1;
                let mut items = ordered.iter();
                while occurrences == 1 {
                    let item = items.next().unwrap().1;
                    occurrences = cards.chars().filter(|&c| c == item).count();
                };
                if occurrences == 3 {
                    3
                }
                else {
                    2
                }
            }
            4 => 1,
            _ => 0
        };
        if joker {
            let jokers = cards.chars().filter(|&c| c == 'J').count();
            match jokers {
                1 => {
                    match value {
                        5 | 0 => value += 1,
                        3 | 2 | 1 => value += 2,
                        _ => {}
                    }
                },
                2 => {
                    match value {
                        4 | 1 => value += 2,
                        2 => value += 3,
                        _ => {}
                    }
                }
                3 => {
                    match value {
                        4 | 3 => value += 2,
                        _ => {}
                    }
                }
                4 => {
                    match value {
                        5 => value += 1,
                        _ => {}
                    }
                }
                0 | _ => {}
            }
        }

        Hand { cards: cards.to_string(), value, joker, sorted }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.value != other.value {
            return self.value.partial_cmp(&other.value);
        }
        let (first, second) = self
            .cards
            .chars()
            .zip(other.cards.chars())
            .filter(|(a, b)| a != b)
            .next()
            .unwrap();
        if self.joker {
            ORDER_2.find(first).unwrap().partial_cmp(&ORDER_2.find(second).unwrap())
        }
        else {
            ORDER.find(first).unwrap().partial_cmp(&ORDER.find(second).unwrap())
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        println!("I compare");
        if self.value != other.value {
            return self.value.cmp(&other.value);
        }
        let (first, second) = self
            .cards
            .chars()
            .zip(other.cards.chars())
            .filter(|(a, b)| a != b)
            .next()
            .unwrap();
        if self.joker {
            ORDER_2.find(first).unwrap().cmp(&ORDER_2.find(second).unwrap())
        }
        else {
            ORDER.find(first).unwrap().cmp(&ORDER.find(second).unwrap())
        }
    }
}
