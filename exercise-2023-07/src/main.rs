use crate::hand::Hand;
use std::fs;

mod hand_type;
mod hand;

fn calculate_game_set(input: String) -> u32 {
    let mut game_set = input
        .lines()
        .filter_map(|line| Hand::new(line).ok())
        .collect::<Vec<_>>();

    game_set.sort();

    println!("{:#?}", game_set);

    let mut sum = 0;
    for (index, hand) in game_set.iter().enumerate() {
        sum += hand.bet * (index as u32 + 1)
    }
    sum
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Hand Set: {}", calculate_game_set(input));
}
