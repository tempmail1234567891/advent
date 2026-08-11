use regex::Regex;
use std::error::Error;
use std::{collections::HashMap, fs};

struct Symbol {
    line: usize,
    index: usize,
}

fn find_symbol(line: &str) -> Option<usize> {
    line.char_indices()
        .find(|(_, c)| !c.is_ascii_digit() && *c != '.')
        .map(|(i, _)| i)
}

fn check_symbol_in_line(
    line: &str,
    start: usize,
    end: usize,
    length: usize,
) -> Result<Option<usize>, &str> {
    match length.checked_add(1) {
        Some(range) => {
            let relative_start = (start % (range)).saturating_sub(1);
            let relative_end = length.min(end % (range) + 1);

            Ok(
                find_symbol(&line[relative_start..relative_end])
                    .map(|index| relative_start + index),
            )
        }
        None => Err("failed to extract first line"),
    }
}

fn validate_rectangle(
    lines: &Vec<&str>,
    start: usize,
    end: usize,
    length: usize,
) -> Option<Symbol> {
    if let Some(range) = length.checked_add(1) {
        let current_line = start / range;

        for i in current_line.saturating_sub(1)..=current_line + 1 {
            if let Some(line) = lines.get(i)
                && let Ok(Some(symbol_index)) = check_symbol_in_line(line, start, end, length)
            {
                return Some(Symbol {
                    line: i,
                    index: symbol_index,
                });
            }
        }
    }
    None
}

fn generate_vector(text: &str) -> Result<Vec<(Symbol, u32)>, Box<dyn Error>> {
    let re: Regex = Regex::new(r"\d+")?;
    let length = text
        .lines()
        .next()
        .ok_or_else(|| "failed to extract first line")?
        .len();
    let lines = text.lines().collect();

    Ok(re
        .find_iter(text)
        .filter_map(|number| {
            validate_rectangle(&lines, number.start(), number.end(), length)
                .map(|symbol| (symbol, number.as_str().parse::<u32>().unwrap()))
        })
        .collect::<Vec<_>>())
}

fn calculate_sum(parts: &[(Symbol, u32)]) -> u32 {
    parts.iter().map(|(_, value)| *value).sum()
}

fn calculate_product(parts: &[(Symbol, u32)]) -> u32 {
    let mut hashmap: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for (symbol, value) in parts {
        hashmap
            .entry((symbol.line, symbol.index))
            .or_default()
            .push(*value);
    }
    hashmap
        .values()
        .filter_map(|values| {
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
    if let Ok(symbols_vector) = generate_vector(&input) {
        let sum: u32 = calculate_sum(&symbols_vector);
        println!("Sum: {}", sum);
        let sum: u32 = calculate_product(&symbols_vector);
        println!("Product: {}", sum);
    }
}
