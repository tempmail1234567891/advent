use crate::board::Board;

mod board;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut board = Board::setup(&input);
    board.walk().unwrap();

    println!("Count: {}", board.calculate_steps(64));
}
