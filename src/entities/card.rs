#[derive(Clone,Copy)]
pub enum Suit {
    Red,
    Green,
    Blue,
    Yellow,
    Black,
}

pub struct Card {
    pub value: u8,
    pub suit: Suit,
}
