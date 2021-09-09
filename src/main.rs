use csv::Reader;
use levenshtein::levenshtein;
use rayon::prelude::*;

mod col_indices;
mod create_tree;
mod user_input;

fn main() {
    let input = user_input::args();
    let (ents_a, vp) = create_tree::tree(&input);
    let mut rdr = Reader::from_path(&input.file_b).expect("creating csv reader");
    let indices = col_indices::cols(&input.file_a_cols);
    let mut ents_b: Vec<create_tree::Entity> = Vec::new();
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

        ents_b.push(create_tree::Entity{i: row, key: cleaned_key});
        row += 1;
    }

    let matches: Vec<Option<&create_tree::Entity>> = ents_b.into_par_iter()
        .map(|ent| {
            let (index, _) = vp.find_nearest(&ent);
            // before returning the WHD match (ents_a), push to its matches field the DHS match (ent)
            if levenshtein(&ent.key, &ents_a[index].key) < input.tolerance { Some(&ents_a[index]) } else { None }
        })
        .collect();
    println!("{:#?}", matches.into_iter().filter(|m| match m {
        Some(_) => true,
        None => false
    }).collect::<Vec<Option<&create_tree::Entity>>>())
}
