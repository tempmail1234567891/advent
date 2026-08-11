use regex::Regex;
use std::error::Error;

pub fn extract_number(line: &str, pattern: &str) -> Result<u32, Box<dyn Error>> {
    let re = Regex::new(pattern)?;
    if let Some(caps) = re.captures(line) {
        let number = caps[1].parse()?;
        Ok(number)
    } else {
        Ok(0)
    }
}