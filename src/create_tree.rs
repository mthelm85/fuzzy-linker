use levenshtein::levenshtein;
use vpsearch::Tree;

use super::entity::*;

impl vpsearch::MetricSpace for Entity {
    type UserData = ();
    type Distance = usize;

    fn distance(&self, other: &Self, _: &Self::UserData) -> Self::Distance {
        levenshtein(&self.key, &other.key)
    }
}

pub fn tree(ents: &Vec<Entity>) -> Tree<Entity> {
    Tree::new(&ents)
}