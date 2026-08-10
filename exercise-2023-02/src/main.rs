use regex::Regex;
use std::error::Error;

fn extract_number(line: &str, pattern: &str) -> Result<u32, Box<dyn Error>> {
    let re = Regex::new(pattern)?;
    if let Some(caps) = re.captures(line) {
        let number = caps[1].parse()?;
        Ok(number)
    } else {
        Ok(0)
    }
}

#[derive(Debug)]
struct Move {
    blue: u32,
    red: u32,
    green: u32,
}

impl Move {
    fn new(record: &str) -> Result<Self, Box<dyn Error>> {
        let blue = extract_number(record, r"(\d+) blue")?;
        let red = extract_number(record, r"(\d+) red")?;
        let green = extract_number(record, r"(\d+) green")?;

        Ok(Self { blue, green , red})
    }

    fn check(self, other: &Move) -> bool {
        self.blue >= other.blue && self.red >= other.red && self.green >= other.green
    }
}

fn main() {
    let first_move = Move::new("2 blue 1 green").unwrap();
    let second_move = Move::new("0 blue 1 green").unwrap();

    println!("{:?}", first_move.check(&second_move));
}
