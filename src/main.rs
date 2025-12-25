use cards::{Card, Suits, Values};
use rand::prelude::SliceRandom;
use rand::rng;

mod cards;

fn main() {
    let mut deck = make_deck();
    deck.shuffle(&mut rng());
    dbg!(make_hand(&mut deck, 50));
    dbg!(deck.len());
}

fn make_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = vec![];
    for suit in Suits::iterator() {
        for value in Values::iterator() {
            deck.push(Card::new(suit.clone(), value.clone()));
        }
    }
    deck
}

fn make_hand(deck: &mut Vec<Card>, size: usize) -> Vec<Card> {
    deck.split_off(size)
}
