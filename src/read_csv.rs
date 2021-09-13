use csv::Reader;

use super::col_indices::*;
use super::create_tree::*;
use super::user_input::*;

pub fn read(input: &Opt, a: bool) -> Vec<Entity> {
    let mut rdr = match a {
        true => Reader::from_path(&input.file_a).expect("creating csv reader"),
        false => Reader::from_path(&input.file_b).expect("creating csv reader")
    };

    let indices = match a {
        true => cols(&input.file_a_cols),
        false => cols(&input.file_b_cols)
    };

    let mut ents: Vec<Entity> = Vec::new();

    for (row, result) in rdr.byte_records().enumerate() {
        let record = result.expect("getting csv record");
        let mut key = "".to_string();
        for i in &indices {
            key.push_str(&String::from_utf8_lossy(record.get(*i).unwrap()))
        }
        let cleaned_key: String = key.chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_ascii_uppercase();

        ents.push(Entity{i: row as u32, key: cleaned_key});
    }

    ents
}