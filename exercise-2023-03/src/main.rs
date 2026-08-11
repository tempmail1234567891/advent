use regex::Regex;
use std::{collections::HashMap, fs};

struct Symbol {
    line: usize,
    index: usize,
}

fn find_symbol(line: &str) -> Option<usize> {
    line.char_indices()
        .filter_map(|(i, c)| (!c.is_ascii_digit() && c != '.').then_some(i))
        .next()
}

fn check_symbol_in_line(line: &str, start: usize, end: usize, length: usize) -> Option<usize> {
    let relative_start = (start % (length + 1)).saturating_sub(1);
    let relative_end = length.min(end % (length + 1) + 1);

    if let Some(index) = find_symbol(&line[relative_start..relative_end]) {
        Some(relative_start + index)
    } else {
        None
    }
}

fn validate_rectangle(
    lines: &Vec<&str>,
    start: usize,
    end: usize,
    length: usize,
) -> Option<Symbol> {
    let current_line = start / (length + 1);

    for i in current_line.saturating_sub(1)..=current_line + 1 {
        if let Some(line) = lines.get(i) {
            if let Some(symbol_index) = check_symbol_in_line(line, start, end, length) {
                return Some(Symbol {
                    line: i,
                    index: symbol_index,
                });
            }
        }
    }
    None
}

fn generate_vector(text: &str) -> Vec<(Symbol, u32)> {
    let re: Regex = Regex::new(r"\d+").unwrap();
    let length = text.lines().next().unwrap().len();
    let lines = text.lines().collect();

    re.find_iter(text)
        .filter_map(|number| {
            if let Some(symbol) = validate_rectangle(&lines, number.start(), number.end(), length) {
                Some((symbol, number.as_str().parse::<u32>().unwrap()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn calculate_sum(parts: &Vec<(Symbol, u32)>) -> u32 {
    parts.iter().map(|(_, value)| *value).sum()
}

fn calculate_product(parts: &Vec<(Symbol, u32)>) -> u32 {
    let mut hashmap: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for (symbol, value) in parts {
        hashmap
            .entry((symbol.line, symbol.index))
            .or_default()
            .push(*value);
    }
    hashmap
        .iter()
        .filter_map(|(_, values)| {
            if values.len() > 1 {
                Some(values.iter().product::<u32>())
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let symbols_vector = generate_vector(&input);
    let sum: u32 = calculate_sum(&symbols_vector);
    println!("Sum: {}", sum);
    let sum: u32 = calculate_product(&symbols_vector);
    println!("Product: {}", sum);
}
