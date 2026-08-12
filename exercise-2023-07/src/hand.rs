use crate::hand_type::{HandError, HandType};
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
pub struct Hand {
    cards: String,
    level: HandType,
    bet: u32,
}

impl Hand {
    pub fn new(record: &str, jokers: &str) -> Result<Self, HandError> {
        let input = record.split_whitespace().take(2).collect::<Vec<_>>();
        if input.len() == 2 {
            let level = HandType::new(input[0], jokers)?;
            let bet = input[1].parse::<u32>()?;
            let cards = String::from(input[0]);
            Ok(Self { cards, level, bet })
        } else {
            Err(HandError::InvalidValue(String::from(
                "record '{record}' must include 2 values",
            )))
        }
    }
    pub fn calculate(&self, rank:u32) -> u32 {
        self.bet * rank
    }
}

impl  Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
         self.level
            .cmp(&other.level)
            .then_with(|| self.cards.cmp(&other.cards))
    }
    
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
