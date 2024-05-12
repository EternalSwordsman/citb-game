use crate::entities::card::Card;
use crate::entities::card::Suit;

pub struct Trick {
    pub played_cards: Vec<(u8, Card)>, // u8 - player ID
    pub player_count: u8,
    pub led_suit: Option<Suit>,
}