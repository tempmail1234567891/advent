use std::collections::HashMap;

#[derive(Debug)]
pub enum TileType {
    Rock,
    Grass,
    Marked(usize),
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn near(&self, other: Point) -> bool {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) == 1
    }
}

pub struct Board {
    start: Point,
    pub board: HashMap<(usize, usize), TileType>,
}

impl Board {
    pub fn setup(input: &str) -> Self {
        let mut start = Point::new(0, 0);
        let mut board = HashMap::new();

        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.char_indices() {
                let tile = match c {
                    'S' => {
                        start = Point::new(i, j);
                        TileType::Marked(0)
                    }
                    '#' => TileType::Rock,
                    _ => TileType::Grass,
                };

                board.insert((i, j), tile);
            }
        }

        Self { start, board }
    }
}
