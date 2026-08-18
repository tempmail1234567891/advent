use crate::tile::Tile;
mod tile;
mod grid;

fn parse_tiles(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .filter_map(|block| {
            let mut lines = block.lines();

            let header = lines.next().unwrap();

            let number: u32 = header
                .strip_prefix("Tile ")
                .unwrap()
                .strip_suffix(":")
                .unwrap()
                .parse()
                .unwrap();

            let tile = lines.collect::<Vec<_>>().join("\n");

            let top = tile
                .lines()
                .next()
                .ok_or_else(|| "missing top line".to_string())
                .unwrap();

            let bottom = tile
                .lines()
                .last()
                .ok_or_else(|| "missing bottom line".to_string())
                .unwrap();

            let left = tile
                .lines()
                .map(|line| line.chars().next().unwrap())
                .collect::<String>();

            let right = tile
                .lines()
                .map(|line| line.chars().last().unwrap())
                .collect::<String>();

            Some(Tile::new(number, &top, &bottom, &left, &right))
        })
        .collect()
}
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let tiles = parse_tiles(&input);
    let length = (tiles.len() as f64).sqrt() as i32;

    let mut grid = grid::Grid::new(tiles, length);

    grid.solve();
    grid.print();
}
