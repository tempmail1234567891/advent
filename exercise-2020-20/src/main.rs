use crate::tile::Tile;
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

            Tile::new(number, &tile).ok()
        })
        .collect()
}
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let tiles = parse_tiles(&input);

    for tile in tiles.iter() {
        for other in tiles.iter() {
            if let Some(location) = tile.order(&other){
                println!("{:?} {:?} {:?}", tile, other, location);
            }
        }
    }
}
