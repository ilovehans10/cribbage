use cards::{Card, Suits, Values};
use rand::prelude::SliceRandom;
use rand::rng;

mod cards;

fn main() {
    let mut deck = make_deck();
    deck.shuffle(&mut rng());
    let mut hand1 = make_hand(&mut deck, 6);
    print_hand(&hand1);
    hand1.swap_remove(3);
    hand1.swap_remove(3);
    print_hand(&hand1);
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
    deck.split_off(deck.len() - size)
}

fn print_hand(deck: &[Card]) {
    for (index, card) in deck.iter().enumerate() {
        if index < deck.len() {
            print!("{}, ", card)
        } else {
            print!("{}", card)
        }
    }
    println!();
}
