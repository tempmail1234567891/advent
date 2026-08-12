use std::{collections::HashMap, num::ParseIntError, str::Chars};
use thiserror::Error;


#[derive(Debug, Error)]
pub enum HandError {
    #[error("invalid value: {0}")]
    InvalidValue(String),

    #[error("failed to parse number")]
    Parsing(#[from] ParseIntError),
}

fn generate_vector_for_hand(hand: Chars, jokers: &str) -> Vec<u32> {
    let mut hashmap: HashMap<char, u32> = HashMap::new();
    let mut found_jokers = 0;

    for character in hand {
        if jokers.contains(character) {
            found_jokers += 1;
        } else {
            *hashmap.entry(character).or_default() += 1;
        }
    }
    let mut vector: Vec<u32> = hashmap.values().copied().collect();

    vector.sort_by_key(|count| std::cmp::Reverse(*count));

    vector.insert(0, found_jokers);
    vector
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl HandType {
    pub fn new(hand: &str, jokers: &str) -> Result<Self, HandError> {
        let vector = generate_vector_for_hand(hand.chars(), jokers);

        if vector.iter().sum::<u32>() != 5 {
            return Err(HandError::InvalidValue(String::from(
                "hand must contain exactly 5 cards",
            )));
        }
        if let Some(jokers) = vector.get(0) {
            if *jokers == 5 {
                return Ok(HandType::FiveKind);
            } else if let Some(first) = vector.get(1) {
                let first = first + jokers;
                if first == 5 {
                    return Ok(HandType::FiveKind);
                } else if first == 4 {
                    return Ok(HandType::FourKind);
                } else if let Some(second) = vector.get(2) {
                    if first == 3 && *second == 2 {
                        return Ok(HandType::FullHouse);
                    } else if first == 3 && *second == 1 {
                        return Ok(HandType::ThreeKind);
                    } else if first == 2 && *second == 2 {
                        return Ok(HandType::TwoPair);
                    } else if first == 2 && *second == 1 {
                        return Ok(HandType::OnePair);
                    } else {
                        return Ok(HandType::HighCard);
                    }
                }
            }
        }
        return Err(HandError::InvalidValue(String::from(
            "Invalid card vector found",
        )));
    }
}
