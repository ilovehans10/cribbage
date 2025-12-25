use self::Suits::*;
use self::Values::*;
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
}

#[derive(Debug)]
pub struct Card {
    suit: Suits,
    value: Values,
}

impl Card {
    pub fn new(suit: Suits, value: Values) -> Card {
        return Card { suit, value };
    }
}
