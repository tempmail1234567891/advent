use regex::Regex;
use std::error::Error;
use std::fs;

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
    red: u32,
    green: u32,
    blue: u32,
}

impl Move {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    fn new_from_record(record: &str) -> Result<Self, Box<dyn Error>> {
        let blue = extract_number(record, r"(\d+) blue")?;
        let red = extract_number(record, r"(\d+) red")?;
        let green = extract_number(record, r"(\d+) green")?;

        Ok(Self { red, green, blue })
    }

    fn check(&self, maximum: &Move) -> bool {
        self.blue <= maximum.blue && self.red <= maximum.red && self.green <= maximum.green
    }
}

#[derive(Debug)]
struct Game {
    number: u32,
    moves: Vec<Move>,
}

impl Game {
    fn new(record: &str) -> Result<Self, Box<dyn Error>> {
        let number = extract_number(record, r"Game (\d+)")?;
        let moves = record
            .split(';')
            .map(|single_move| Move::new_from_record(single_move))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { number, moves })
    }

    fn check(&self, maximum: &Move) -> bool {
        self.moves.iter().all(|m| m.check(maximum))
    }
}

fn main() {
    let maximum_move = Move::new(12, 13, 14);
    let input = fs::read_to_string("input.txt").unwrap();

    let games = input.lines().filter_map(|line| Game::new(line).ok());
    let sum: u32 = games.filter(|game| game.check(&maximum_move)).map(|game| game.number).sum();
    println!("Sum: {sum}");
}
