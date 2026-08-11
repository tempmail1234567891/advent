use crate::utils::{self, UtilsError};

use super::cube;

#[derive(Debug)]
pub struct Game {
    pub number: u32,
    pub cubes: Vec<cube::Cube>,
}

impl Game {
    pub fn new(record: &str) -> Result<Self, UtilsError> {
        let number = utils::extract_number(record, r"Game (\d+)")?;
        let cubes = record
            .split(';')
            .map(|single_move| cube::Cube::new_from_record(single_move))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { number, cubes })
    }

    pub fn smaller_than(&self, cube: &cube::Cube) -> bool {
        self.cubes.iter().all(|m| m.smaller_than(cube))
    }

    pub fn minimum_cube(&self) -> Option<cube::Cube> {
        let red = self.cubes.iter().map(|cube| cube.red).max()?;
        let green = self.cubes.iter().map(|cube| cube.green).max()?;
        let blue = self.cubes.iter().map(|cube| cube.blue).max()?;

        Some(cube::Cube { red, green, blue })
    }
}