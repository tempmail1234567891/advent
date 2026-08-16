use std::collections::HashMap;

use crate::board::{Board, TileType};

mod board;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut board = Board::setup(&input);
    board.print();
    board.walk();

    board.print();
}
