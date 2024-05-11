use crate::card::Card;
use crate::card::Suit;

#[derive(Clone)]
enum ResearchState {
    P1,
    P2,
    P3,
    P4,
    P5,
    Unplayed,
}

pub struct ResearchBoard {
    display: Vec<Vec<Option<ResearchState>>>,
}

impl ResearchBoard {
    pub fn reset(&mut self, player_count: u8) {
        let max_card_value: u8 = match player_count {
            5 => 9,
            4 => 8,
            3 => 6,
            2 => 5,
            _ => 0, // TODO - this should error
        };

        for suit in 0..4 {    // TODO - can we use the suit enum directly here?
            for value in 1..=max_card_value {
                self.display[suit][value as usize] = Some(ResearchState::Unplayed);  // TODO - don't like this cast
            }
        }
    }

    pub fn check_if_playable(&mut self, card: &Card) -> bool {
        match &self.display[card.suit as usize][card.value as usize] {
            None => false,
            Some(ResearchState::Unplayed) => true,
            _ => false,
        }  // TODO - is there a better way to do this?
    }

    pub fn mark_played(&mut self, card: &Card, player_id: u8) {
        if self.check_if_playable(card) == false {
            // TODO - error
        }
        let player_state: ResearchState = match player_id {
            1 => ResearchState::P1,
            2 => ResearchState::P2,
            3 => ResearchState::P3,
            4 => ResearchState::P4,
            5 => ResearchState::P5,
            _ => ResearchState::Unplayed,
        };

        self.display[card.suit as usize][card.value as usize] = Some(player_state);
    }

    pub fn check_if_trump_played(&mut self) -> bool {
        let red_row = &self.display[Suit::Red as usize];
        for state in red_row {
            match state {
                None => false,
                Some(ResearchState::Unplayed) => false,
                _ => return true,
            };
        }
        false
    }
}

// =========================================
//             TEST SECTION
// =========================================

#[cfg(test)]
mod research_board_tests {
    use super::*;

    #[test]
    fn reset_5p() {
        let mut research_board = ResearchBoard {display: vec![vec![None; 10]; 4]};
        let unplayed_card = Card {
            suit: Suit::Yellow,
            value: 9,
        };
        let invalid_card = Card {
            suit: Suit::Red,
            value: 0,
        };
        research_board.reset(5);
        assert_eq!(true, research_board.check_if_playable(&unplayed_card));
        assert_eq!(false, research_board.check_if_playable(&invalid_card));
    }

    #[test]
    fn reset_4p() {
        let mut research_board = ResearchBoard {display: vec![vec![None; 10]; 4]};
        let unplayed_card = Card {
            suit: Suit::Yellow,
            value: 8,
        };
        let invalid_card = Card {
            suit: Suit::Red,
            value: 9,
        };
        research_board.reset(4);
        assert_eq!(true, research_board.check_if_playable(&unplayed_card));
        assert_eq!(false, research_board.check_if_playable(&invalid_card));
    }

    #[test]
    fn mark_card_played() {
        let mut research_board = ResearchBoard {display: vec![vec![None; 10]; 4]};
        let card_to_play = Card {
            suit: Suit::Yellow,
            value: 9,
        };
        let player_id = 1;
        research_board.reset(5);
        assert_eq!(true, research_board.check_if_playable(&card_to_play));
        research_board.mark_played(&card_to_play, player_id);
        assert_eq!(false, research_board.check_if_playable(&card_to_play));
    }

    #[test]
    fn check_if_trump_played() {
        let mut research_board = ResearchBoard {display: vec![vec![None; 10]; 4]};
        let card_to_play = Card {
            suit: Suit::Red,
            value: 9,
        };
        let player_id = 1;
        research_board.reset(5);
        assert_eq!(false, research_board.check_if_trump_played());
        research_board.mark_played(&card_to_play, player_id);
        assert_eq!(true, research_board.check_if_trump_played());
    }

    #[test]
    #[should_panic]
    fn wildly_invalid_values() {
        let mut research_board = ResearchBoard {display: vec![vec![None; 10]; 4]};
        let card_to_play = Card {
            suit: Suit::Yellow,
            value: 50,
        };
        assert_eq!(false, research_board.check_if_playable(&card_to_play));
    }
}