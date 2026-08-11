use regex::Regex;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UtilsError {
    #[error("regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("failed to parse integer: {0}")]
    ParseInt(#[from] ParseIntError),
}

pub fn extract_number(line: &str, pattern: &str) -> Result<u32, UtilsError> {
    let re = Regex::new(pattern)?;
    if let Some(caps) = re.captures(line) {
        let number = caps[1].parse()?;
        Ok(number)
    } else {
        Ok(0)
    }
}