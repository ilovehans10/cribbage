use cards::Card;
use rand::prelude::SliceRandom;
use rand::rng;
use scoring::Scorer;

mod cards;
mod scoring;

fn main() {
    let mut deck = Card::make_deck();
    deck.shuffle(&mut rng());
    let mut hand1 = Card::make_hand(&mut deck, 6);
    Card::print_hand(&hand1);
    hand1.swap_remove(3);
    hand1.swap_remove(3);
    Card::print_hand(&hand1);
    for solver in Scorer::rules_for_show() {
        println!("Score from {}s: {}", solver.name, (solver.rule)(&hand1));
    }
}
