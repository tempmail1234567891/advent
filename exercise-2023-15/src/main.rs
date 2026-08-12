use std::fs;

mod container;
mod lense;
mod game;


fn calculate_sum(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.split(','){
        sum += game::hash(line) as u32;
    }
    sum
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Found: {}", calculate_sum(&input));

    let mut g = game::Game::new();
    for line in input.split(','){
        g.execute_operation(line).unwrap();
    }

    println!("Lense Power: {}", g.calculate_game());
}
