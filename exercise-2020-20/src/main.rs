use crate::tile::Tile;
mod direction;
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
                .ok_or_else(|| "missing top line".to_string()).unwrap();

            let bottom = tile
                .lines()
                .last()
                .ok_or_else(|| "missing bottom line".to_string()).unwrap();

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
    let tiles = parse_tiles(&input);

    for tile in tiles.iter() {
        for other in tiles.iter() {
            for direction in direction::ALL {
                if tile.id != other.id && let Some(location) = tile.is_neighbors(&other, &direction) {
                    println!("{:?} {:?} {:?} {:?}", tile.id, other.id, direction, location);
                }
            }
        }
    }
}
