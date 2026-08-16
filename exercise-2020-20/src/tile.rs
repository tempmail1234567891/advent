#[derive(Debug)]
pub struct Tile {
    number: u32,
    top: u16,
    bottom: u16,
    left: u16,
    right: u16,
}

#[derive(Debug)]
pub enum Location {
    Above,
    Below,
    OnLeft,
    OnRight,
}

fn convert(line: String) -> Result<u16, String> {
    if line.len() > 16 {
        Err("line length must be smaller than 16".to_string())
    } else {
        let mut number = 0;
        for (i, character) in line.char_indices() {
            match character {
                '.' => {}
                '#' => {
                    number |= 1 << i;
                }
                _ => return Err("invalid character found".to_string()),
            }
        }
        Ok(number)
    }
}

fn is_match(a: u16, b: u16) -> bool {
    a == b || !a == b || a == !b
}

impl Tile {
    pub fn new(number: u32, tile: &str) -> Result<Self, String> {
        let top = tile
            .lines()
            .next()
            .ok_or_else(|| "missing top line".to_string())?
            .to_string();
        let top = convert(top)?;
        let bottom = tile
            .lines()
            .last()
            .ok_or_else(|| "missing bottom line".to_string())?
            .to_string();
        let bottom = convert(bottom)?;

        let left = convert(
            tile.lines()
                .map(|line| line.chars().next().unwrap())
                .collect(),
        )?;

        let right = convert(
            tile.lines()
                .map(|line| line.chars().last().unwrap())
                .collect(),
        )?;

        Ok(Self {
            number,
            top,
            bottom,
            left,
            right,
        })
    }

    pub fn order(&self, other: &Tile) -> Option<Location> {
        if self.number == other.number {
            None
        } else if is_match(self.top, other.bottom) || is_match(self.top, other.top) {
            Some(Location::Above)
        } else if is_match(self.bottom, other.bottom) || is_match(self.bottom, other.top) {
            Some(Location::Below)
        } else if is_match(self.right, other.right) || is_match(self.right, other.left) {
            Some(Location::OnRight)
        } else if is_match(self.left, other.right) || is_match(self.left, other.left) {
            Some(Location::OnLeft)
        } else {
            None
        }
    }
}
