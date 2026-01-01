use self::Suits::*;
use self::Values::*;
use std::fmt::Display;
use std::slice::Iter;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Values {
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

impl Values {
    pub fn iterator() -> Iter<'static, Values> {
        static VALUES: [Values; 13] = [
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
}

#[derive(Debug, Clone)]
pub struct Card {
    suit: Suits,
    value: Values,
}

impl Card {
    pub fn new(suit: Suits, value: Values) -> Card {
        Card { suit, value }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.suit.short_name(), self.value.short_name())
    }
}
