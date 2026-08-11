use std::fs;
use exercise_2023_02::cube;
use exercise_2023_02::game;

fn main() {
    let maximum_cube = cube::Cube::new(12, 13, 14);
    let input = fs::read_to_string("input.txt").unwrap();

    let games = input.lines().filter_map(|line| game::Game::new(line).ok());

    let sum: u32 = games
        .clone()
        .filter(|game| game.possible_game(&maximum_cube))
        .map(|game| game.number)
        .sum();
    println!("Sum: {sum}");

    let power: u32 = games
        .filter_map(|game| game.minimum_cube())
        .map(|cube| cube.power())
        .sum();
    println!("Power: {power}");
}
