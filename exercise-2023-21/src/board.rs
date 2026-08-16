use std::collections::HashMap;

type BoardType = HashMap<Point, TileType>;

#[derive(Debug, PartialEq)]
pub enum TileType {
    Rock,
    Grass,
    Marked(usize),
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn near(&self, board: &BoardType) -> Vec<Point> {
        let mut next_points = vec![];
        
        let point = Point::new(self.x - 1, self.y);
        if let Some(tile) = board.get(&point) && *tile == TileType::Grass {
            next_points.push(point);
        }
        next_points
    }
}

pub struct Board {
    start: Point,
    pub board: BoardType,
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

                board.insert(Point::new(i, j), tile);
            }
        }

        Self { start, board }
    }
}
