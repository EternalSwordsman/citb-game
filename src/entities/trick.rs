use crate::card::Card;
use crate::entities::player::Player;

pub struct Trick {
    pub played_cards: Vec<(Player, Card)>,
}