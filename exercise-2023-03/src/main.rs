use regex::Regex;
use std::fs;

fn extract_rectangle(text: &str, start: usize, end: usize, length: usize) -> String {
    let mut rectangle = String::new();
    if length <= start {
        rectangle += &text[(start - length).saturating_sub(1)..(end - length + 1)];
    }
    rectangle += &text[start.saturating_sub(1)..start];
    rectangle += &text[end..text.len().min(end.saturating_add(1))];

    if end + length <= text.len() {
        rectangle += &text[(start + length).saturating_sub(1)..text.len().min(end + length + 1)];
    }

    rectangle
}

fn calculate_sum(text: &str) -> u32 {
    let re = Regex::new(r"\d+").unwrap();
    let length = text.lines().next().unwrap().len() + 1;
    let matches: Vec<_> = re
        .find_iter(text)
        .map(|caps| {
            let rectangle = extract_rectangle(text, caps.start(), caps.end(), length);
            if rectangle.chars().any(|c| !c.is_ascii_digit() && c != '.' && c!='\n') {
                caps.as_str().parse::<u32>().unwrap()
            } else {
                println!("Invalid: {}, {rectangle}", caps.as_str());
                0
            }
        })
        .collect();

    matches.iter().sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let sum: u32 = calculate_sum(&input);

    println!("Match: {}", sum);
}
