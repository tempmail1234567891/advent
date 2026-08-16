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
    pub board: Vec<Vec<TileType>>,
}

impl Board {
    pub fn setup(input: &str) -> Self {
        let mut board = vec![];
        let mut point = Point { x: 0, y: 0 };
        for (i, line) in input.lines().enumerate() {
            let row = line
                .char_indices()
                .map(|(j, c)| match c {
                    'S' => {
                        point = Point::new(i, j);
                        TileType::Marked(0)
                    }
                    '#' => TileType::Rock,
                    _ => TileType::Grass,
                })
                .collect::<Vec<_>>();

            board.push(row);
        }
        Self {
            start: point,
            board,
        }
    }
}
