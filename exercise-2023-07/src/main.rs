use std::fs;

mod basic_game;
mod hand;
mod hand_type;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Hand Set: {}", basic_game::calculate_game_set(input));
}
