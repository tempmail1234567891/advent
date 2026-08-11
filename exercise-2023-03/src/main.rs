use regex::Regex;
use std::fs;

fn extract_from_line(line: &str, start: usize, end: usize, length: usize) -> &str {
    let relative_start = (start % (length + 1)).saturating_sub(1);
    let relative_end = length.min(end % (length + 1) + 1);

    &line[relative_start..relative_end]
}

fn extract_rectangle(lines: &Vec<&str>, start: usize, end: usize, length: usize) -> Result<String, String> {
    let mut rectangle = String::new();

    let current_line = start / (length + 1);

    if current_line >= lines.len(){
        return Err(String::from("search for substring outside of the existing lines"));
    }

    for i in current_line.saturating_sub(1)..=current_line + 1 {
        if let Some(line) = lines.get(i) {
            rectangle +=  extract_from_line(line, start, end, length)
        }
    }

    Ok(rectangle)
}

fn calculate_sum(text: &str) -> u32 {
    let re = Regex::new(r"\d+").unwrap();
    let length = text.lines().next().unwrap().len();
    let lines = text.lines().collect();

    let matches = re
        .find_iter(text)
        .filter_map(|caps| {
            let rectangle = extract_rectangle(&lines, caps.start(), caps.end(), length).ok()?;
            if rectangle.chars().any(|c| !c.is_ascii_digit() && c != '.') {
                Some(caps.as_str().parse::<u32>().unwrap())
            } else {
                None
            }
        }).collect::<Vec<_>>();

    matches.into_iter().sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let sum: u32 = calculate_sum(&input);

    println!("Match: {}", sum);
}
