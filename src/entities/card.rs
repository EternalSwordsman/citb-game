#[derive(Clone,Copy,PartialEq)]
pub enum Suit {
    Red,
    Green,
    Blue,
    Yellow,
    Black,
}

#[derive(Clone,Copy)]
pub struct Card {
    pub value: u8,
    pub suit: Suit,
}
