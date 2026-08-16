use crate::board::Board;

mod board;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let board = Board::setup(&input);
    println!("{:#?}", board.board[2][1]);
}
