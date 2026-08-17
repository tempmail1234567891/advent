use crate::direction::{Direction, RelativeLocation};

#[derive(Debug, Clone)]
pub struct Card {
    top: String,
    bottom: String,
    left: String,
    right: String,
}

impl Card {
    fn new(top: &str, bottom: &str, left: &str, right: &str) -> Self {
        Self {
            top: top.to_string(),
            bottom: bottom.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub id: u32,
    direction: Direction,
    source: Card,
    parsed: Card,
}

impl Tile {
    pub fn new(id: u32, top: &str, bottom: &str, left: &str, right: &str) -> Result<Self, String> {
        let card = Card::new(top, bottom, left, right);
        Ok(Self {
            id,
            direction: Direction::Default,
            source: card,
            parsed: Card::new(top, bottom, left, right),
        })
    }

    pub fn set_direction(&mut self, direction: Direction) {
        let parsed = match direction {
            Direction::Default => Card::new(
                &self.source.top,
                &self.source.bottom,
                &self.source.left,
                &self.source.right,
            ),
            Direction::XFlip => Card::new(
                &self.source.bottom,
                &self.source.top,
                &self.source.left.chars().rev().collect::<String>(),
                &self.source.right.chars().rev().collect::<String>(),
            ),
            Direction::YFlip => Card::new(
                &self.source.top.chars().rev().collect::<String>(),
                &self.source.bottom.chars().rev().collect::<String>(),
                &self.source.right,
                &self.source.left,
            ),
            Direction::XYFlip => Card::new(
                &self.source.bottom.chars().rev().collect::<String>(),
                &self.source.top.chars().rev().collect::<String>(),
                &self.source.right.chars().rev().collect::<String>(),
                &self.source.left.chars().rev().collect::<String>(),
            ),
        };

        self.direction = direction;
        self.parsed = parsed;
    }

    fn check_match(
        &self,
        current: &String,
        straigt: &String,
        opposite: &String,
    ) -> Option<Direction> {
        if current == straigt {
            Some(Direction::Default)
        } else if current == opposite {
            Some(Direction::XFlip)
        } else if *current == straigt.chars().rev().collect::<String>() {
            Some(Direction::YFlip)
        } else if *current == opposite.chars().rev().collect::<String>() {
            Some(Direction::XYFlip)
        } else {
            None
        }
    }

    pub fn is_neighbors_by_location(
        &self,
        other: &Tile,
        location: &RelativeLocation,
    ) -> Option<Direction> {
        match location {
            RelativeLocation::Above => {
                self.check_match(&self.parsed.bottom, &other.source.top, &other.source.bottom)
            }
            RelativeLocation::Below => {
                self.check_match(&self.parsed.top, &other.source.bottom, &other.source.top)
            }
            RelativeLocation::OnLeft => {
                self.check_match(&self.parsed.left, &other.source.right, &other.source.left)
            }
            RelativeLocation::OnRight => {
                self.check_match(&self.parsed.right, &other.source.left, &other.source.right)
            }
        }
    }
}
