use crate::hand_type::HandType;
use std::fs;

mod hand_type;

#[derive(Debug)]
struct Hand {
    hand: HandType,
    bet: u32,
}

impl Hand {
    fn new(record: &str) -> Result<Self, hand_type::HandError> {
        let input = record.split_whitespace().take(2).collect::<Vec<_>>();
        if input.len() == 2 {
            let hand = HandType::new(input[0])?;
            let bet = input[1].parse::<u32>()?;
            Ok(Self { hand, bet })
        } else {
            Err(hand_type::HandError::InvalidValue(String::from(
                "record '{record}' must include 2 values",
            )))
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    for line in input.lines() {
        let hand = Hand::new(line).unwrap();
        println!("{:?}", hand);
    }
}
