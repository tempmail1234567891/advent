use crate::hand_type::{HandError, HandType};

#[derive(Debug)]
pub struct Hand {
    pub cards: String,
    pub level: HandType,
    pub bet: u32,
}

impl Hand {
    pub fn new(record: &str) -> Result<Self, HandError> {
        let input = record.split_whitespace().take(2).collect::<Vec<_>>();
        if input.len() == 2 {
            let level = HandType::new(input[0])?;
            let bet = input[1].parse::<u32>()?;
            let cards = String::from(input[0]);
            Ok(Self { cards, level, bet })
        } else {
            Err(HandError::InvalidValue(String::from(
                "record '{record}' must include 2 values",
            )))
        }
    }
}
