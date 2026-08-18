use std::ops::Not;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Field {
    pub v: Vec<bool>,
}

impl Field {
    fn new(input: &str) -> Self {
        Self {
            v: input
                .chars()
                .map(|c| match c {
                    '.' => false,
                    _ => true,
                })
                .collect(),
        }
    }

    fn to_string(&self) -> String {
        self.v
            .iter()
            .map(|v| match v {
                true => "1",
                false => "0",
            })
            .collect()
    }
}
impl Not for Field {
    type Output = Field;

    fn not(self) -> Self::Output {
        Field {
            v: self.v.into_iter().rev().collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub id: u32,
    pub top: Field,
    pub bottom: Field,
    pub left: Field,
    pub right: Field,
}

impl Tile {
    pub fn new(id: u32, top: &str, bottom: &str, left: &str, right: &str) -> Self {
        Self {
            id,
            top: Field::new(top),
            bottom: Field::new(bottom),
            left: Field::new(left),
            right: Field::new(right),
        }
    }
    pub fn rotate(self) -> Tile {
        Tile {
            id: self.id,
            top: self.left,
            bottom: self.right,
            left: self.bottom,
            right: self.top,
        }
    }

    pub fn flip_x(self) -> Tile {
        Tile {
            id: self.id,
            top: self.bottom,
            bottom: self.top,
            left: !self.left,
            right: !self.right,
        }
    }

    pub fn flip_y(self) -> Tile {
        Tile {
            id: self.id,
            top: !self.top,
            bottom: !self.bottom,
            left: self.right,
            right: self.left,
        }
    }

    pub fn orientations(self) -> Vec<Tile> {
        vec![
            self.clone(),
            self.clone().flip_x(),
            self.clone().flip_y(),
            self.clone().flip_x().flip_y(),
            self.clone().rotate(),
            self.clone().rotate().rotate().rotate(),
            self.clone().rotate().flip_x(),
            self.clone().rotate().rotate().rotate().flip_x(),
        ]
    }

    pub fn print(&self) -> Vec<String> {
        let width = self.top.v.len();
        let height = self.left.v.len();

        let mut lines = Vec::with_capacity(height);

        lines.push(self.top.to_string());

        for row in 1..height - 1 {
            let left = if self.left.v[row] { '1' } else { '0' };
            let right = if self.right.v[row] { '1' } else { '0' };

            let inner_width = width - 2;
            let text = self.id.to_string();

            let content = if row == (height / 2) {
                format!("{:^width$}", text, width = inner_width)
            } else {
                " ".repeat(inner_width)
            };

            lines.push(format!("{}{}{}", left, content, right));
        }

        lines.push(self.bottom.to_string());

        lines
    }
}
