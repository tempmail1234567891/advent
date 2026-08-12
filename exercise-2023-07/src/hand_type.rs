use std::{collections::HashMap, str::Chars};

fn generate_vector_for_hand(hand: Chars) -> Vec<u32> {
    let mut hashmap: HashMap<char, u32> = HashMap::new();
    for character in hand {
        *hashmap.entry(character).or_default() += 1;
    }
    let mut vector:Vec<u32> = hashmap.values().copied().collect();

    vector.sort_by_key(|count| std::cmp::Reverse(*count));
    vector
}

#[derive(Debug)]
pub enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    pub fn new(hand: &str) -> Result<Self,&str> {
        let vector = generate_vector_for_hand(hand.chars());

        if vector.iter().sum::<u32>() != 5 {
            return Err("hand must contain exactly 5 cards");
        }
        if vector[0] == 5 {
            Ok(HandType::FiveKind)
        } else if vector[0] == 4 {
            Ok(HandType::FourKind)
        } else if vector[0] == 3 && vector[1] == 2 {
            Ok(HandType::FullHouse)
        } else if vector[0] == 3 && vector[1] == 1 {
            Ok(HandType::ThreeKind)
        } else if vector[0] == 2 && vector[1] == 2 {
            Ok(HandType::TwoPair)
        } else if vector[0] == 2 && vector[1] == 1 {
            Ok(HandType::OnePair)
        } else {
            Ok(HandType::HighCard)
        }
    }
}
