use crate::cards::Card;
use itertools::Itertools;

pub struct Scorer {
    pub name: String,
    pub rule: Box<dyn Fn(&Vec<Card>) -> usize>,
}

impl Scorer {
    pub fn rules_for_show() -> std::vec::Vec<Scorer> {
        vec![Scorer::solver_15(), Scorer::solver_pair()]
    }

    pub(super) fn solver_15() -> Scorer {
        Scorer {
            name: String::from("15"),
            rule: Box::new(|deck: &Vec<Card>| {
                deck.iter()
                    .powerset()
                    .map(|x| x.iter().fold(0, |a, b| a + b.to_cribbage_value()))
                    .filter(|x| x == &15)
                    .count()
                    * 2
            }),
        }
    }

    pub(super) fn solver_pair() -> Scorer {
        Scorer {
            name: String::from("Pair"),
            rule: Box::new(|deck: &Vec<Card>| {
                deck.iter()
                    .combinations(2)
                    .filter(|x| x[0].rank == x[1].rank)
                    .count()
                    * 2
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cards::{Card, Suits};

    use super::Scorer;

    #[test]
    fn scorer_15() {
        assert_eq!(
            2,
            (Scorer::solver_15().rule)(&vec![
                Card::new(Suits::Hearts, crate::cards::Ranks::Five),
                Card::new(Suits::Hearts, crate::cards::Ranks::Ten),
            ])
        )
    }

    #[test]
    fn bad_scorer_15() {
        assert_eq!(
            0,
            (Scorer::solver_15().rule)(&vec![
                Card::new(Suits::Hearts, crate::cards::Ranks::Four),
                Card::new(Suits::Hearts, crate::cards::Ranks::Jack),
            ])
        )
    }

    #[test]
    fn scorer_double_15() {
        assert_eq!(
            4,
            (Scorer::solver_15().rule)(&vec![
                Card::new(Suits::Hearts, crate::cards::Ranks::Five),
                Card::new(Suits::Spades, crate::cards::Ranks::Five),
                Card::new(Suits::Hearts, crate::cards::Ranks::Ten),
            ])
        )
    }

    #[test]
    fn scorer_tripple_15() {
        assert_eq!(
            6,
            (Scorer::solver_15().rule)(&vec![
                Card::new(Suits::Hearts, crate::cards::Ranks::Five),
                Card::new(Suits::Hearts, crate::cards::Ranks::Ten),
                Card::new(Suits::Spades, crate::cards::Ranks::Ten),
                Card::new(Suits::Clubs, crate::cards::Ranks::Ten),
            ])
        )
    }

    #[test]
    fn scorer_pair() {
        assert_eq!(
            2,
            (Scorer::solver_pair().rule)(&vec![
                Card::new(Suits::Hearts, crate::cards::Ranks::Five),
                Card::new(Suits::Spades, crate::cards::Ranks::Five),
            ])
        )
    }

    #[test]
    fn bad_scorer_pair() {
        assert_eq!(
            0,
            (Scorer::solver_pair().rule)(&vec![
                Card::new(Suits::Hearts, crate::cards::Ranks::Jack),
                Card::new(Suits::Spades, crate::cards::Ranks::King),
            ])
        )
    }

    #[test]
    fn scorer_double_pair() {
        assert_eq!(
            4,
            (Scorer::solver_pair().rule)(&vec![
                Card::new(Suits::Hearts, crate::cards::Ranks::Five),
                Card::new(Suits::Spades, crate::cards::Ranks::Five),
                Card::new(Suits::Hearts, crate::cards::Ranks::Four),
                Card::new(Suits::Spades, crate::cards::Ranks::Four),
            ])
        )
    }

    #[test]
    fn scorer_triple_pair() {
        assert_eq!(
            6,
            (Scorer::solver_pair().rule)(&vec![
                Card::new(Suits::Hearts, crate::cards::Ranks::Five),
                Card::new(Suits::Spades, crate::cards::Ranks::Five),
                Card::new(Suits::Clubs, crate::cards::Ranks::Five),
            ])
        )
    }
}
