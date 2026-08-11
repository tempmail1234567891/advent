use std::fs;
use crate::hand_type::HandType;

mod hand_type;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    for line in input.lines(){
        let hand = HandType::new(line.split(' ').next().unwrap());
        println!("{:?}", hand);
    }

}
