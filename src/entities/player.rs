use crate::card::Card;
use crate::card::Suit;

pub struct Player {
    pub id: u8,
    pub hand: Vec<Card>,
    pub available_suits: Vec<Suit>,
    pub score: u8,
    pub name: String,
    pub bid: u8,
    pub tricks_this_round: u8,
}
