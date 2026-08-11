use crate::utils::{self, UtilsError};

#[derive(Debug)]
pub struct Cube {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Cube {
    pub fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    pub fn new_from_record(record: &str) -> Result<Self, UtilsError> {
        let blue = utils::extract_number(record, r"(\d+) blue")?;
        let red = utils::extract_number(record, r"(\d+) red")?;
        let green = utils::extract_number(record, r"(\d+) green")?;

        Ok(Self { red, green, blue })
    }

    pub fn smaller_than(&self, cube: &Cube) -> bool {
        self.blue <= cube.blue && self.red <= cube.red && self.green <= cube.green
    }

    pub fn power(&self) -> u32 {
        self.blue * self.red * self.green
    }
}

