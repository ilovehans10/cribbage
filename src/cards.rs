use self::Ranks::*;
use self::Suits::*;
use itertools::{Itertools, Position};
use std::fmt::Display;
use std::slice::Iter;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suits {
    Diamonds,
    Hearts,
    Clubs,
    Spades,
}

impl Suits {
    pub fn iterator() -> Iter<'static, Suits> {
        static SUITS: [Suits; 4] = [Diamonds, Hearts, Clubs, Spades];
        SUITS.iter()
    }

    pub fn short_name(&self) -> String {
        match self {
            Diamonds => "♦",
            Hearts => "♥",
            Clubs => "♣",
            Spades => "♠",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Ranks {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Ranks {
    pub fn iterator() -> Iter<'static, Ranks> {
        static VALUES: [Ranks; 13] = [
            Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King,
        ];
        VALUES.iter()
    }

    pub fn short_name(&self) -> String {
        match self {
            Ace => "A",
            Two => "2",
            Three => "3",
            Four => "4",
            Five => "5",
            Six => "6",
            Seven => "7",
            Eight => "8",
            Nine => "9",
            Ten => "10",
            Jack => "J",
            Queen => "Q",
            King => "K",
        }
        .to_string()
    }

    pub fn to_cribbage_value(&self) -> usize {
        match self {
            Jack => 10,
            Queen => 10,
            King => 10,
            _ => self.to_rank_value(),
        }
    }

    pub fn to_rank_value(&self) -> usize {
        match self {
            Ace => 1,
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            Ten => 10,
            Jack => 11,
            Queen => 12,
            King => 13,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub rank: Ranks,
    pub suit: Suits,
}

impl Card {
    pub fn new(suit: Suits, value: Ranks) -> Card {
        Card { suit, rank: value }
    }

    pub fn to_cribbage_value(&self) -> usize {
        self.rank.to_cribbage_value()
    }

    pub fn to_rank_value(&self) -> usize {
        self.rank.to_rank_value()
    }

    pub fn make_deck() -> Vec<Card> {
        let mut deck: Vec<Card> = vec![];
        for suit in Suits::iterator() {
            for value in Ranks::iterator() {
                deck.push(Card::new(suit.clone(), value.clone()));
            }
        }
        deck
    }

    pub fn make_hand(deck: &mut Vec<Card>, size: usize) -> Vec<Card> {
        deck.split_off(deck.len() - size)
    }

    pub fn print_hand(deck: &[Card]) {
        for (deck_position, card) in deck.iter().with_position() {
            match deck_position {
                Position::Last => print!("{}", card),
                _ => print!("{}, ", card),
            }
        }
        println!();
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.suit.short_name(), self.rank.short_name())
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::{Card, Suits};

    #[test]
    fn card_suit_ne() {
        assert_ne!(
            Card::new(Suits::Hearts, crate::cards::Ranks::Five),
            Card::new(Suits::Spades, crate::cards::Ranks::Five),
        );
    }

    #[test]
    fn card_rank_ne() {
        assert_ne!(
            Card::new(Suits::Hearts, crate::cards::Ranks::Five),
            Card::new(Suits::Hearts, crate::cards::Ranks::Ten),
        );
    }

    #[test]
    fn card_rank_cmp() {
        assert!(
            Card::new(Suits::Hearts, crate::cards::Ranks::Five)
                < Card::new(Suits::Hearts, crate::cards::Ranks::Ten),
        );
    }

    #[test]
    fn card_both_cmp() {
        assert!(
            Card::new(Suits::Hearts, crate::cards::Ranks::Five)
                < Card::new(Suits::Diamonds, crate::cards::Ranks::Ten),
        );
    }
}
