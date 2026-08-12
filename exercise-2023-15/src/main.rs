use std::fs;

mod container;
mod lense;

fn hash(line: &str) -> u8 {
    let mut number: u32 = 0;

    for character in line.chars() {
        let code = character as u8;
        number += code as u32;
        number *= 17;
        number %= 256;
    }
    number as u8
}

fn calculate_sum(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.split(','){
        sum += hash(line) as u32;
    }
    sum
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Found: {}", calculate_sum(&input));
}
