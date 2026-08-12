use crate::container::Box;

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

struct Game {
    boxes: Vec<Box>
}

impl Game {
    fn execute_operation(&self, command: &str) {
        
    }
}