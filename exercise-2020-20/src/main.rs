use crate::{grid::Point, tile::Tile};
mod direction;
mod grid;
mod tile;

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

            Tile::new(number, &top, &bottom, &left, &right).ok()
        })
        .collect()
}
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut iter = parse_tiles(&input).into_iter();

    let first = iter.next().unwrap();
    let rest: Vec<Tile> = iter.collect();

    let mut grid = grid::Grid::new(first, rest);

    grid.organize(&Point::new(0, 0));

    grid.print();
}
