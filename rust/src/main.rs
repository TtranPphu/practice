mod leetcode;
use macro_utils::comprehension;

fn main() {
    let deck = comprehension! {
        format!("{}{}", rank, suit)
        for rank in ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"]
        for suit in ["♠", "♣", "♥", "♦"]
    };
    for card in deck {
        println!("{}", card);
    }
}
