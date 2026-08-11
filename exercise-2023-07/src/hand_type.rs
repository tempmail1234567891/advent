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
    pub fn new(hand: &str) -> Self {
        let vector = generate_vector_for_hand(hand.chars());

        if vector[0] == 5 {
            HandType::FiveKind
        } else if vector[0] == 4 {
            HandType::FourKind
        } else if vector[0] == 3 && vector[1] == 2 {
            HandType::FullHouse
        } else if vector[0] == 3 && vector[1] == 1 {
            HandType::ThreeKind
        } else if vector[0] == 2 && vector[1] == 2 {
            HandType::TwoPair
        } else if vector[0] == 2 && vector[1] == 1 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}
