use std::collections::HashMap;

use crate::{container::Box, lense::Lense};
use regex::Regex;
use thiserror::Error;

pub fn hash(line: &str) -> u8 {
    let mut number: u32 = 0;

    for character in line.chars() {
        let code = character as u8;
        number += code as u32;
        number *= 17;
        number %= 256;
    }
    number as u8
}

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Unknown command found")]
    CommandNotFound,
    
    #[error("Box not found")]
    BoxNotFound,
    
    #[error("Parsing number failed")]
    Parsing(#[from] std::num::ParseIntError)
}

#[derive(Debug)]
pub struct Game {
    boxes: HashMap<u8, Box>,
}

impl Game {
    pub fn new() -> Self {
        let boxes: HashMap<u8, Box> = (0..=255).map(|i| (i, Box::new())).collect();
        Self { boxes }
    }

    fn check_add_operation(&mut self, command: &str) -> Result<bool, GameError> {
        let add_pattern = Regex::new(r"\A([a-z]+)=(\d)\z").unwrap();

        if let Some(captures) = add_pattern.captures(command) {
            let hash_number = hash(&captures[1]);
            let lense_value = captures[2].parse::<u8>()?;

            if let Some(current_box) = self.boxes.get_mut(&hash_number) {
                current_box.add(Lense::new(String::from(&captures[1]), lense_value));
                return Ok(true);
            }
            return Err(GameError::BoxNotFound);
        }
        Ok(false)
    }

    fn check_remove_operation(&mut self, command: &str) -> Result<bool, GameError> {
        let remove_pattern = Regex::new(r"\A([a-z]+)-\z").unwrap();

        if let Some(captures) = remove_pattern.captures(command) {
            let hash_number = hash(&captures[1]);

            if let Some(current_box) = self.boxes.get_mut(&hash_number) {
                current_box.remove(&captures[1]);
                return Ok(true);
            }
            return Err(GameError::BoxNotFound);
        }
        Ok(false)
    }

    pub fn execute_operation(&mut self, command: &str) -> Result<(), GameError> {
        if !self.check_add_operation(command)? && !self.check_remove_operation(command)? {
            Err(GameError::CommandNotFound)
        } else {
            Ok(())
        }
    }

    pub fn calculate_game(&self) -> usize {
        let mut boxes = self.boxes.iter().collect::<Vec<_>>();
        boxes.sort_by_key(|(index, _)| *index);

        let mut sum = 0;
        for (i, current_box) in boxes {
            for (j, lense) in current_box.lenses().iter().enumerate() {
                sum += ((*i as usize) + 1) * (j + 1) * (lense.value as usize);
            }
        }
        sum
    }
}
