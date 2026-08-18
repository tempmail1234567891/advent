use crate::tile::Tile;
use std::collections::HashMap;

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

        for tile in self.tiles.clone() {
            for oriented in tile.clone().orientations() {
                if !self.does_fit(&oriented, x, y) {
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

    pub fn calculate(&self) -> u64 {
        let corner1 = self.board.get(&(0, 0)).map(|t| t.id).unwrap_or(0) as u64;
        let corner2 = self
            .board
            .get(&(0, self.size - 1))
            .map(|t| t.id)
            .unwrap_or(0) as u64;
        let corner3 = self
            .board
            .get(&(self.size - 1, 0))
            .map(|t| t.id)
            .unwrap_or(0) as u64;
        let corner4 = self
            .board
            .get(&(self.size - 1, self.size - 1))
            .map(|t| t.id)
            .unwrap_or(0) as u64;
        corner1 * corner2 * corner3 * corner4
    }

    fn does_fit(&self, tile: &Tile, x: i32, y: i32) -> bool {
        if self.board.contains_key(&(x, y)) {
            false
        } else {
            if let Some(above) = self.board.get(&(x, y - 1))
                && (above.bottom != tile.top || above.id == tile.id)
            {
                false
            } else if let Some(bellow) = self.board.get(&(x, y + 1))
                && (bellow.top != tile.bottom || bellow.id == tile.id)
            {
                false
            } else if let Some(on_left) = self.board.get(&(x - 1, y))
                && (on_left.right != tile.left || on_left.id == tile.id)
            {
                false
            } else if let Some(on_right) = self.board.get(&(x + 1, y))
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
