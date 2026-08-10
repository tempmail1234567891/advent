use regex::Regex;
use std::error::Error;

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

    pub fn new_from_record(record: &str) -> Result<Self, Box<dyn Error>> {
        let blue = extract_number(record, r"(\d+) blue")?;
        let red = extract_number(record, r"(\d+) red")?;
        let green = extract_number(record, r"(\d+) green")?;

        Ok(Self { red, green, blue })
    }

    pub fn smaller_than(&self, cube: &Cube) -> bool {
        self.blue <= cube.blue && self.red <= cube.red && self.green <= cube.green
    }

    pub fn power(&self) -> u32 {
        self.blue * self.red * self.green
    }
}

pub fn extract_number(line: &str, pattern: &str) -> Result<u32, Box<dyn Error>> {
    let re = Regex::new(pattern)?;
    if let Some(caps) = re.captures(line) {
        let number = caps[1].parse()?;
        Ok(number)
    } else {
        Ok(0)
    }
}
