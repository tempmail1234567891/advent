use crate::{direction::RelativeLocation, tile::Tile};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn near(&self) -> Vec<(Point, RelativeLocation)> {
        vec![
            (Point::new(self.x - 1, self.y), RelativeLocation::OnLeft),
            (Point::new(self.x + 1, self.y), RelativeLocation::OnRight),
            (Point::new(self.x, self.y - 1), RelativeLocation::Below),
            (Point::new(self.x, self.y + 1), RelativeLocation::Above),
        ]
    }
}

#[derive(Debug)]
pub struct Grid {
    board: HashMap<Point, Tile>,
}

impl Grid {
    pub fn new(tile: Tile) -> Self {
        Self {
            board: HashMap::from([(Point { x: 0, y: 0 }, tile)]),
        }
    }
    pub fn organize(&mut self, tiles: Vec<Tile>, source: &Point) {
        for (i, _) in tiles.iter().enumerate() {
            let mut remaining = tiles.clone();
            let tile = remaining.remove(i);

            if let Some(new_point) = self.insert_around(tile, source) {
                self.organize(remaining, &new_point);
            }
        }
    }

    pub fn insert_around(&mut self, mut other: Tile, source: &Point) -> Option<Point> {
        if self.board.contains_key(&source) {
            let tile = self.board.get(source).unwrap();
            if tile.id == other.id {
                return None;
            }
            for (next, location) in source.near() {
                if !self.board.contains_key(&next)
                    && let Some(direction) = tile.is_neighbors_by_location(&other, &location)
                {
                    other.set_direction(direction);
                    self.board.insert(next, other);
                    return Some(next);
                }
            }
        }
        None
    }

    pub fn print(&self) {
        let min_x = self.board.keys().map(|p| p.x).min().unwrap();
        let max_x = self.board.keys().map(|p| p.x).max().unwrap();
        let min_y = self.board.keys().map(|p| p.y).min().unwrap();
        let max_y = self.board.keys().map(|p| p.y).max().unwrap();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                match self.board.get(&Point::new(x, y)) {
                    Some(value) => print!("{:>5}", value.id),
                    None => print!("{:>5}", "."),
                }
            }
            println!();
        }
    }
}
