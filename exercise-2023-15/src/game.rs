use crate::container::Box;
use regex::Regex;

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

pub struct Game {
    boxes: Vec<Box>
}

impl Game {
    pub fn new() ->Self {
        Self { boxes: vec![] }
    }

    pub fn execute_operation(&self, command: &str) {
        let add_pattern = regex::Regex::new(r"\A([a-z]+)=(\d)\z").unwrap();
        let remove_pattern = regex::Regex::new(r"\A([a-z]+)-\z").unwrap();

        if let Some(captures) = add_pattern.captures(command) {
            println!("Add {} to {}", &captures[2], &captures[1]);
        }

        
        if let Some(captures) = remove_pattern.captures(command) {
            println!("Remove {}", &captures[1]);
        }
    }
}