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
struct Cube {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cube {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    fn new_from_record(record: &str) -> Result<Self, Box<dyn Error>> {
        let blue = extract_number(record, r"(\d+) blue")?;
        let red = extract_number(record, r"(\d+) red")?;
        let green = extract_number(record, r"(\d+) green")?;

        Ok(Self { red, green, blue })
    }

    fn smaller_than(&self, cube: &Cube) -> bool {
        self.blue <= cube.blue && self.red <= cube.red && self.green <= cube.green
    }

    fn power(&self) -> u32 {
        self.blue * self.red * self.green
    }
}

#[derive(Debug)]
struct Game {
    number: u32,
    cubes: Vec<Cube>,
}

impl Game {
    fn new(record: &str) -> Result<Self, Box<dyn Error>> {
        let number = extract_number(record, r"Game (\d+)")?;
        let cubes = record
            .split(';')
            .map(|single_move| Cube::new_from_record(single_move))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { number, cubes })
    }

    fn smaller_than(&self, cube: &Cube) -> bool {
        self.cubes.iter().all(|m| m.smaller_than(cube))
    }

    fn minimum_cube(&self) -> Option<Cube> {
        let red = self.cubes.iter().map(|cube| cube.red).max()?;
        let green = self.cubes.iter().map(|cube| cube.green).max()?;
        let blue = self.cubes.iter().map(|cube| cube.blue).max()?;

        Some(Cube { red, green, blue })
    }
}

fn main() {
    let maximum_cube = Cube::new(12, 13, 14);
    let input = fs::read_to_string("input.txt").unwrap();

    let games = input.lines().filter_map(|line| Game::new(line).ok());

    let sum: u32 = games
        .clone()
        .filter(|game| game.smaller_than(&maximum_cube))
        .map(|game| game.number)
        .sum();
    println!("Sum: {sum}");

    let power: u32 = games
        .filter_map(|game| game.minimum_cube())
        .map(|cube| cube.power())
        .sum();
    println!("Power: {power}");
}
