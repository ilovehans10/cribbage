use cards::{Card, Ranks, Suits};
use itertools::{Itertools, Position};
use rand::prelude::SliceRandom;
use rand::rng;
use scoring::Scorer;

mod cards;
mod scoring;

fn main() {
    let mut deck = make_deck();
    deck.shuffle(&mut rng());
    let mut hand1 = make_hand(&mut deck, 6);
    print_hand(&hand1);
    hand1.swap_remove(3);
    hand1.swap_remove(3);
    print_hand(&hand1);
    dbg!(deck.len());
    for solver in Scorer::rules_for_show() {
        println!("Score from {}s: {}", solver.name, (solver.rule)(&hand1));
    }
}

fn make_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = vec![];
    for suit in Suits::iterator() {
        for value in Ranks::iterator() {
            deck.push(Card::new(suit.clone(), value.clone()));
        }
    }
    deck
}

fn make_hand(deck: &mut Vec<Card>, size: usize) -> Vec<Card> {
    deck.split_off(deck.len() - size)
}

fn print_hand(deck: &[Card]) {
    for (deck_position, card) in deck.iter().with_position() {
        match deck_position {
            Position::Last => print!("{}", card),
            _ => print!("{}, ", card),
        }
    }
    println!();
}
