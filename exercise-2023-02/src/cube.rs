use crate::utils::{self, UtilsError};
use std::cmp::Ordering;

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

    pub fn power(&self) -> u32 {
        self.blue * self.red * self.green
    }
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.blue == other.blue && self.red == other.red && self.green == other.green
    }
}

impl PartialOrd for Cube {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let blue = self.blue.cmp(&other.blue);
        let red = self.red.cmp(&other.red);
        let green = self.green.cmp(&other.green);

        if blue == Ordering::Equal && red == Ordering::Equal && green == Ordering::Equal {
            Some(Ordering::Equal)
        } else if blue != Ordering::Greater
            && red != Ordering::Greater
            && green != Ordering::Greater
        {
            Some(Ordering::Less)
        } else if blue != Ordering::Less && red != Ordering::Less && green != Ordering::Less {
            Some(Ordering::Greater)
        } else {
            None // incomparable
        }
    }
}
