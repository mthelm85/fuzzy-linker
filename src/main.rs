use csv::Writer;
use indicatif::ParallelProgressIterator;
use levenshtein::levenshtein;
use rayon::prelude::*;
use serde::Serialize;

mod col_indices;
mod create_tree;
mod user_input;
mod read_csv;

#[derive(Debug, Serialize)]
struct CsvRecord {
    row_a: u32,
    row_b: u32,
}

fn main() {
    let input = user_input::args();
    println!("Building tree...");
    let (ents_a, vp) = create_tree::tree(&input, true);
    let ents_b: Vec<create_tree::Entity> = read_csv::read(&input, false);
    println!("Searching for potential matches...");
    let end = ents_b.len() as u64;
    let matches: Vec<Option<CsvRecord>> = ents_b.into_par_iter()
        .progress_count(end)
        .map(|ent| {
            let (index, _) = vp.find_nearest(&ent);
            if levenshtein(&ent.key, &ents_a[index].key) < (input.tolerance * ent.key.len() as f32) as usize {
                Some(CsvRecord {
                    row_a: ents_a[index].i + 2,
                    row_b: ent.i + 2
                })
            } else { None }
        })
        .collect();

        let mut wtr = Writer::from_path(&input.output).expect("creating CSV writer");
    matches.iter()
        .for_each(|r| {
            if let Some(r) = r { wtr.serialize(r).expect("serializing CSV record") }
        });
    wtr.flush().expect("flushing CSV writer");
}
