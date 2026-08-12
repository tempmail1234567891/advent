use std::fs;

mod game;
mod hand;
mod hand_type;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Hand Set: {}", game::calculate_game_set(input));
}
