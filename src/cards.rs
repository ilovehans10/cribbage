use self::Ranks::*;
use self::Suits::*;
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

    pub fn to_value(&self) -> usize {
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
            Jack => 10,
            Queen => 10,
            King => 10,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Card {
    pub suit: Suits,
    pub value: Ranks,
}

impl Card {
    pub fn new(suit: Suits, value: Ranks) -> Card {
        Card { suit, value }
    }

    pub fn to_value(&self) -> usize {
        self.value.to_value()
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.suit.short_name(), self.value.short_name())
    }
}
