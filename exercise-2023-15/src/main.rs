use std::fs;

fn hash(line: &str) -> u32 {
    let mut number: u32 = 0;

    for character in line.chars() {
        let code = character as u8;
        number += code as u32;
        number *= 17;
        number %= 256;
    }
    number
}

fn calculate_sum(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.split(','){
        sum += hash(line);
    }
    sum
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Found: {}", calculate_sum(&input));
}
