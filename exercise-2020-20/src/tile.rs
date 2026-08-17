use crate::direction;
use crate::direction::{Direction, RelativeLocation};

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Card {
    top: u16,
    bottom: u16,
    left: u16,
    right: u16,
}

impl Card {
    fn new(top: u16, bottom: u16, left: u16, right: u16) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }

    fn convert(line: &str) -> Result<u16, String> {
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
}

#[derive(Debug)]
pub struct Tile {
    pub id: u32,
    direction: Direction,
    source: Card,
    parsed: Card,
}

impl Tile {
    pub fn new(id: u32, top: &str, bottom: &str, left: &str, right: &str) -> Result<Self, String> {
        let card = Card::new(
            Card::convert(top)?,
            Card::convert(bottom)?,
            Card::convert(left)?,
            Card::convert(right)?,
        );
        Ok(Self {
            id,
            direction: Direction::Default,
            source: card,
            parsed: card.clone(),
        })
    }

    pub fn set_direction(&mut self, direction: Direction) {
        let parsed = match direction {
            Direction::Default => Card::new(
                self.source.top,
                self.source.bottom,
                self.source.left,
                self.source.right,
            ),
            Direction::XFlip => Card::new(
                self.source.bottom,
                self.source.top,
                !self.source.left,
                !self.source.right,
            ),
            Direction::YFlip => Card::new(
                !self.source.top,
                !self.source.bottom,
                self.source.right,
                self.source.left,
            ),
            Direction::XYFlip => Card::new(
                !self.source.bottom,
                !self.source.top,
                !self.source.right,
                !self.source.left,
            ),
        };

        self.direction = direction;
        self.parsed = parsed;
    }

    fn check_match(&self, current: u16, straigt: u16, opposite: u16) -> Option<Direction> {
        if current == straigt {
            Some(Direction::Default)
        } else if current == opposite {
            Some(Direction::XFlip)
        } else if current == !straigt {
            Some(Direction::YFlip)
        } else if current == !opposite {
            Some(Direction::XYFlip)
        } else {
            None
        }
    }

    fn is_neighbors_by_location(
        &self,
        other: &Tile,
        location: &RelativeLocation,
    ) -> Option<Direction> {
        match location {
            RelativeLocation::Above => {
                self.check_match(self.parsed.bottom, other.source.top, other.source.bottom)
            }
            RelativeLocation::Below => {
                self.check_match(self.parsed.top, other.source.bottom, other.source.top)
            }
            RelativeLocation::OnLeft => {
                self.check_match(self.parsed.left, other.source.right, other.source.left)
            }
            RelativeLocation::OnRight => {
                self.check_match(self.parsed.right, other.source.left, other.source.right)
            }
        }
    }

    pub fn is_neighbors(&self, other: &Tile) -> Option<(RelativeLocation, Direction)> {
        for direction in direction::ALL {
            if self.id != other.id
                && let Some(location) = self.is_neighbors_by_location(&other, &direction)
            {
                return Some((direction, location));
            }
        }
        None
    }
}
