use regex::{Regex};
use std::{fs};


fn calculate_sum(text: &str) -> u32 {
    let re = Regex::new(r"\d+").unwrap();
    let matches: Vec<_> = re
        .find_iter(text)
        .map(|caps| {
            println!(
                "Match: {}, start: {}, end: {}",
                caps.as_str(),
                caps.start(),
                caps.end()
            );
            caps.as_str().parse::<u32>().unwrap()
        })
        .collect();

    matches.iter().sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let sum: u32 = calculate_sum(&input);

    println!("Match: {}", sum);
}
