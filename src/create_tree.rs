use csv::Reader;
use levenshtein::levenshtein;
use vpsearch::Tree;

use super::col_indices::*;
use super::user_input::*;

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

pub fn tree(input: &Opt) -> (Vec<Entity>, Tree<Entity>) {
    let mut rdr = Reader::from_path(&input.file_a).expect("creating csv reader");
    let indices = cols(&input.file_a_cols);
    let mut ents: Vec<Entity> = Vec::new();
    let mut row = 0;

    for result in rdr.byte_records() {
        let record = result.expect("getting csv record");
        let mut key = "".to_string();
        for i in &indices {
            key.push_str(&String::from_utf8_lossy(record.get(*i).unwrap()))
        }
        let cleaned_key: String = key.chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_ascii_uppercase();

        ents.push(Entity{i: row, key: cleaned_key});
        row += 1;
    }

    (ents.clone(), Tree::new(&ents))
}