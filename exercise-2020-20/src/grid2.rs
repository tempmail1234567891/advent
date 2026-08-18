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

    pub fn solve(&mut self) -> bool {
        if self.tiles.len() == 0 {
            return true;
        }

        let index = self.board.len() as i32;
        let x = index % self.size;
        let y = index / self.size;
        let point = Point::new(x, y);

        for tile in self.tiles.clone() {
            for oriented in tile.clone().orientations() {
                if !self.does_fit(&oriented, &point) {
                    continue;
                }

                self.tiles.retain(|t| t.id != tile.id);

                self.board.insert((x, y), oriented);

                if self.solve() {
                    return true;
                }

                self.board.remove(&(x, y));
                self.tiles.push(tile.clone());
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

    pub fn print(&self) {
        for y in 0..self.size {
            let tiles: Vec<_> = (0..self.size)
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
                        None => print!("{}", " ".repeat(10)),
                    }
                }
                println!();
            }
        }
    }
}
