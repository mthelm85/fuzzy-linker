use levenshtein::levenshtein;
use vpsearch::Tree;

use super::user_input::*;
use super::read_csv::*;

#[derive(Clone, Debug)]
pub struct Entity {
    pub i: u32,
    pub key: String
}

impl vpsearch::MetricSpace for Entity {
    type UserData = ();
    type Distance = usize;

    fn distance(&self, other: &Self, _: &Self::UserData) -> Self::Distance {
        levenshtein(&self.key, &other.key)
    }
}

pub fn tree(input: &Opt, a: bool) -> (Vec<Entity>, Tree<Entity>) {
    let ents = read(&input, a);
    (ents.clone(), Tree::new(&ents))
}