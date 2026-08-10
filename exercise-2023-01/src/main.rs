use std::collections::HashMap;
use std::fs;
use std::str::Lines;
use std::sync::LazyLock;

const DIGITS: LazyLock<HashMap<&str, u32>> = LazyLock::new(|| {
    HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ])
});

fn order_substrings(line: &str) -> Vec<u32> {
    let mut substrings = DIGITS
        .iter()
        .map(|(text, value)| line.match_indices(*text).map(|(index, _)| (index, *value)))
        .flatten()
        .collect::<Vec<_>>();

    substrings.sort_by_key(|(index, _)| *index);

    substrings.into_iter().map(|(_, value)| value).collect()
}

fn extract_number(line: &str) -> Option<u32> {
    let results = order_substrings(line);
    if !results.is_empty() {
        let first = results.first()?;
        let second = results.last()?;
        Some(*first * 10 + *second)
    } else {
        Some(0)
    }
}

fn calculate_sum(lines: Lines) -> u32 {
    lines.filter_map(extract_number).sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let sum = calculate_sum(input.lines());
    println!("found: {sum}");
}
