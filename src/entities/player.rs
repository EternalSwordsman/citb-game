use crate::card::Card;
use crate::card::Suit;

pub struct Player {
    id: u8,
    hand: Vec<Card>,
    available_suits: Vec<Suit>,
    score: u8,
    name: String,
    bid: u8,
    tricks_this_round: u8,
}
