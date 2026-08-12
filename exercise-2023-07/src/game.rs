use std::{collections::HashMap, sync::LazyLock};

use crate::hand::Hand;

const JOKERS: &str = "1";
const MAPPER: LazyLock<HashMap<char,char>> = LazyLock::new(|| HashMap::from([
    ('A', 'F'),
    ('K', 'D'),
    ('Q', 'C'),
    ('J', '1'),
    ('T', 'A'),
]));

fn parse_input(input: String) -> String {
    input
        .chars()
        .map(|c| *MAPPER.get(&c).unwrap_or(&c))
        .collect()
}

pub fn calculate_game_set(input: String) -> u32 {
    let mut game_set = parse_input(input)
        .lines()
        .filter_map(|line| Hand::new(line, JOKERS).ok())
        .collect::<Vec<_>>();

    game_set.sort();

    let mut sum = 0;
    for (index, hand) in game_set.iter().enumerate() {
        sum += hand.calculate(index as u32 + 1)
    }
    sum
}