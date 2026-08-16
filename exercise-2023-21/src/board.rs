use std::collections::HashMap;

pub type BoardType = HashMap<Point, TileType>;

#[derive(Debug, PartialEq)]
pub enum TileType {
    Rock,
    Grass,
    Marked(usize),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn near(&self) -> Vec<Point> {
        vec![
            Point::new(self.x + 1, self.y),
            Point::new(self.x.saturating_sub(1), self.y),
            Point::new(self.x, self.y + 1),
            Point::new(self.x, self.y.saturating_sub(1)),
        ]
    }
}

pub struct Board {
    start: Point,
    length: usize,
    pub board: BoardType,
}

impl Board {
    pub fn setup(input: &str) -> Self {
        let mut start = Point::new(0, 0);
        let mut board = HashMap::new();
        let mut length = 0;

        for (i, line) in input.lines().enumerate() {
            length = line.len() -1 ;
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

        Self { length, start, board }
    }
    pub fn walk(&mut self) -> Result<(), String> {
        walk(&self.start, &mut self.board)
    }

    pub fn print(&self) {
        for x in 0..=self.length {
            for y in 0..=self.length {
                let point = Point { x, y };

                let character = match self.board.get(&point) {
                    Some(TileType::Marked(mark)) => mark.to_string(),
                    Some(TileType::Rock) => "#".to_string(),
                    Some(TileType::Grass) => ".".to_string(),
                    None => "?".to_string(),
                };
                print!("{}", character);
            }
            println!();
        }
    }
}

fn walk(point: &Point, board: &mut BoardType) -> Result<(), String> {
    println!("{:?}", point);
    let mark = match board.get(point) {
        Some(TileType::Marked(mark)) => *mark,
        Some(_) => return Err("given tile is not marked".to_string()),
        None => return Err("given point not in board".to_string()),
    };

    let neighbors = point.near();

    for next in neighbors {
        if let Some(TileType::Grass) = board.get(&next) {
            board.insert(next, TileType::Marked(mark + 1));
            walk(&next, board).ok();
        }
    }

    Ok(())
}
