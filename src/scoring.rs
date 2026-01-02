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
                    .map(|x| x.iter().fold(0, |a, b| a + b.to_value()))
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
                    .filter(|x| x[0].value == x[1].value)
                    .count()
                    * 2
            }),
        }
    }
}
