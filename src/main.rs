use csv::Writer;
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
    let (ents_a, vp) = create_tree::tree(&input, true);
    let ents_b: Vec<create_tree::Entity> = read_csv::read(&input, false);

    let matches: Vec<Option<CsvRecord>> = ents_b.into_par_iter()
        .map(|ent| {
            let (index, _) = vp.find_nearest(&ent);
            if levenshtein(&ent.key, &ents_a[index].key) < input.tolerance {
                Some(CsvRecord {
                    row_a: ents_a[index].i,
                    row_b: ent.i
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
