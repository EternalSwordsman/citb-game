use crate::card::Card;
use crate::entities::player::Player;

pub struct Trick {
    played_cards: Vec<(Player, Card)>,
}