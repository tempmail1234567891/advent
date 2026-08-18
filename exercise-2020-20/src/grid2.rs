use crate::tile2::Tile;
use std::collections::HashMap;

#[derive(Debug, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn near(&self) -> Vec<Point> {
        vec![
            Point::new(self.x - 1, self.y),
            Point::new(self.x + 1, self.y),
            Point::new(self.x, self.y - 1),
            Point::new(self.x, self.y + 1),
        ]
    }
}

#[derive(Debug)]
pub struct Grid {
    board: HashMap<(i32, i32), Tile>,
    tiles: Vec<Tile>,
    size: i32,
}

impl Grid {
       pub fn new(tiles: Vec<Tile>, size: i32) -> Self {
        Self {
            board: HashMap::new(),
            size,
            tiles,
        }
    }

    pub fn solve(&mut self, target: &Point) -> bool {
        if self.tiles.len() == 0 {
            return true;
        }
        
        for tile in self.tiles.clone() {
            if self.does_fit(&tile, target) {
                self.tiles.retain(|t| t.id != tile.id);
                self.board.insert((target.x, target.y), tile.clone());

                for point in target.near() {
                    // if self.validate_point(&point){
                        if self.solve(&point){
                            return true;
                        }
                    // }
                }

                self.board.remove(&(target.x, target.y));
                self.tiles.append(&mut tile.orientations());
            }
        }
        false
    }

    fn does_fit(&self, tile: &Tile, target: &Point) -> bool {
        if self.board.contains_key(&(target.x, target.y)) {
            false
        } else {
            if let Some(above) = self.board.get(&(target.x, target.y - 1))
                && (above.bottom != tile.top || above.id == tile.id)
            {
                false
            } else if let Some(bellow) = self.board.get(&(target.x, target.y + 1))
                && (bellow.top != tile.bottom || bellow.id == tile.id)
            {
                false
            } else if let Some(on_left) = self.board.get(&(target.x - 1, target.y))
                && (on_left.right != tile.left || on_left.id == tile.id)
            {
                false
            } else if let Some(on_right) = self.board.get(&(target.x + 1, target.y))
                && (on_right.left != tile.right || on_right.id == tile.id)
            {
                false
            } else {
                true
            }
        }
    }

    fn board_range(&self) -> (i32, i32, i32, i32) {
        let min_x = self.board.keys().map(|p| p.0).min().unwrap_or(0);
        let max_x = self.board.keys().map(|p| p.0).max().unwrap_or(0);
        let min_y = self.board.keys().map(|p| p.1).min().unwrap_or(0);
        let max_y = self.board.keys().map(|p| p.1).max().unwrap_or(0);
        (min_x, max_x, min_y, max_y)
    }

    fn validate_point(&self, target: &Point) -> bool {
        let (min_x, max_x, min_y, max_y) = self.board_range();

        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;

        if width == self.size && (target.x < min_x || target.x > max_x){
            false
        }
        else if height == self.size && (target.y < min_y || target.y > max_y){
            false
        }
        else {
            true
        }
    }

    pub fn print(&self) {
        let (min_x, max_x, min_y, max_y) = self.board_range();

        for y in min_y..=max_y {
            let tiles: Vec<_> = (min_x..=max_x)
                .map(|x| self.board.get(&(x, y)).map(|tile| tile.print()))
                .collect();

            let height = tiles
                .iter()
                .filter_map(|t| t.as_ref())
                .map(|t| t.len())
                .max()
                .unwrap_or(0);

            for row in 0..height {
                for tile in &tiles {
                    match tile {
                        Some(lines) => print!("{}", lines[row]),
                        None => print!("{}", " ".repeat(10)), // tile width
                    }
                }
                println!();
            }
        }
    }
}
