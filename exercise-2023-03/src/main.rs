use regex::Regex;
use std::fs;

fn extract_rectangle(lines: &Vec<&str>, start: usize, end: usize, length: usize) -> String {
    let mut rectangle = String::new();

    let index = start / (length + 1);
    let relative_start = (start % (length + 1)).saturating_sub(1);
    let relative_end = length.min(end % (length + 1) + 1);

    if index > 0 {
        rectangle += &lines[index - 1][relative_start..relative_end];
    }
    rectangle += &lines[index][relative_start..relative_end];

    if index < lines.len() - 1 {
        rectangle += &lines[index + 1][relative_start..relative_end];
    }

    rectangle
}

fn calculate_sum(text: &str) -> u32 {
    let re = Regex::new(r"\d+").unwrap();
    let length = text.lines().next().unwrap().len();
    let lines = text.lines().collect();

    let matches: Vec<_> = re
        .find_iter(text)
        .map(|caps| {
            let rectangle = extract_rectangle(&lines, caps.start(), caps.end(), length);
            if rectangle
                .chars()
                .any(|c| !c.is_ascii_digit() && c != '.')
            {
                caps.as_str().parse::<u32>().unwrap()
            } else {
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
