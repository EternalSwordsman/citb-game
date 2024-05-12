use crate::entities::trick::Trick;
use crate::entities::card::Card;
use crate::entities::card::Suit;

const TRUMP_SUIT: Suit = Suit::Red;

pub fn determine_higher_card(first_card: (u8, Card), second_card: (u8, Card), led_suit: Suit) -> (u8, Card) {
    // First, check if one card is trump and the other is not
    if first_card.1.suit == TRUMP_SUIT && second_card.1.suit != TRUMP_SUIT {
        return first_card
    }
    if second_card.1.suit == TRUMP_SUIT && first_card.1.suit != TRUMP_SUIT {
        return second_card
    }
    // Next, determine if one card is led suit and other is not
    if first_card.1.suit == led_suit && second_card.1.suit != led_suit {
        return first_card
    }
    if second_card.1.suit == led_suit && first_card.1.suit != led_suit {
        return second_card
    }
    // If both cards are led suit, compare value
    if second_card.1.value > first_card.1.value {
        return second_card
    }
    return first_card
}

// Returns the player_id of the winner of the trick
pub fn determine_trick_winner(trick: &Trick) -> u8 {
    let mut index = 0;
    let mut winning_card = trick.played_cards[index];
    index = index + 1;
    while index < trick.played_cards.len() {
        winning_card = determine_higher_card(winning_card, trick.played_cards[index], trick.led_suit.unwrap());
        index = index + 1;
    }
    return winning_card.0;
}

// Play a card to the trick
// Return: Option<u8> - returns the player_id of the winner of the trick,
// or "None" if there is no winner yet
pub fn play_card(trick: &mut Trick, card: Card, player_id: u8) -> Option<u8> {
    if trick.led_suit.is_none() {
        trick.led_suit = Some(card.suit);
    }
    trick.played_cards.push((player_id, card));
    if trick.played_cards.len() >= trick.player_count as usize {
        return Some(determine_trick_winner(&trick));
    }
    else {
        return None
    }
}


#[cfg(test)]
mod play_card_tests {
    use super::*;
    use crate::entities::card::Suit;

    #[test]
    fn determine_higher_card_test() {
        let test_card1 = Card{value: 7, suit: Suit::Blue};
        let test_card2 = Card{value: 5, suit: Suit::Blue};
        let test_card3 = Card{value: 9, suit: Suit::Blue};
        let test_card4 = Card{value: 1, suit: Suit::Red};
        let test_card5 = Card{value: 1, suit: Suit::Blue};
        let test_card6 = Card{value: 9, suit: Suit::Yellow};

        let higher_card = determine_higher_card((1, test_card1), (2, test_card2), Suit::Blue);
        assert_eq!(1, higher_card.0);
        let higher_card = determine_higher_card((1, test_card3), (2, test_card4), Suit::Blue);
        assert_eq!(2, higher_card.0);
        let higher_card = determine_higher_card((1, test_card5), (2, test_card6), Suit::Blue);
        assert_eq!(1, higher_card.0);
    }

    #[test]
    fn determine_trick_winner_test() {
        let test_card1 = Card{value: 1, suit: Suit::Blue};
        let test_card2 = Card{value: 5, suit: Suit::Blue};
        let test_card3 = Card{value: 7, suit: Suit::Blue};
        let test_card4 = Card{value: 9, suit: Suit::Yellow};
        let mut test_trick = Trick{played_cards: Vec::new(), player_count: 4, led_suit: None};

        let trick_winner = play_card(&mut test_trick, test_card1, 1);
        assert_eq!(None, trick_winner);
        let trick_winner = play_card(&mut test_trick, test_card2, 2);
        assert_eq!(None, trick_winner);
        let trick_winner = play_card(&mut test_trick, test_card3, 3);
        assert_eq!(None, trick_winner);
        let trick_winner = play_card(&mut test_trick, test_card4, 4);
        assert_eq!(Some(3), trick_winner);
    }

    #[test]
    fn play_card_test() {
        let mut test_trick = Trick{played_cards: Vec::new(),player_count: 5, led_suit: None};
        let test_card = Card{value: 5, suit: Suit::Blue};

        let trick_winner = play_card(&mut test_trick, test_card, 1);
        assert_eq!(None, trick_winner);
        assert_eq!(1, test_trick.played_cards.len());
    }
}