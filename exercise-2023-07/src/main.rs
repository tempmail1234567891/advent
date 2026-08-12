use crate::hand_type::HandType;
use std::fs;

mod hand_type;

#[derive(Debug)]
struct Hand {
    cards: String,
    level: HandType,
    bet: u32,
}

impl Hand {
    fn new(record: &str) -> Result<Self, hand_type::HandError> {
        let input = record.split_whitespace().take(2).collect::<Vec<_>>();
        if input.len() == 2 {
            let level = HandType::new(input[0])?;
            let bet = input[1].parse::<u32>()?;
            Ok(Self {
                cards: String::from(input[0]),
                level,
                bet,
            })
        } else {
            Err(hand_type::HandError::InvalidValue(String::from(
                "record '{record}' must include 2 values",
            )))
        }
    }
}

fn calculate_game_set(input: String) -> u32 {
    let mut game_set = input
        .lines()
        .filter_map(|line| Hand::new(line).ok())
        .collect::<Vec<_>>();

    game_set.sort_by(|a, b| a.level.cmp(&b.level).then_with(|| b.cards.cmp(&a.cards)));

    let mut sum = 0;
    for (index, hand) in game_set.iter().enumerate() {
        sum += hand.bet * (index as u32 + 1)
    }
    sum
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Hand Set: {}",calculate_game_set(input));
}
