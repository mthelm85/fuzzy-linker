use anyhow::{ Context, ensure, Result };
use csv::Writer;
use indicatif::ParallelProgressIterator;
use levenshtein::levenshtein;
use rayon::prelude::*;
use serde::Serialize;

mod col_indices;
mod create_tree;
mod errors;
mod user_input;
mod read_csv;

#[derive(Debug, Serialize)]
struct CsvRecord {
    row_a: u32,
    row_b: u32,
}

fn main() -> Result<(), anyhow::Error> {
    let input = user_input::args();
    ensure!(input.tolerance < 1.0 && input.tolerance > 0.0, errors::Error::ToleranceError { t: input.tolerance });
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

        let mut wtr = Writer::from_path(&input.output).context("Error: failed to create CSV writer")?;
    matches.iter()
        .for_each(|r| {
            if let Some(r) = r {
                match wtr.serialize(r) {
                    Ok(rcrd) => rcrd,
                    Err(e) => eprintln!("{}", e)
                }
            }
        });
    wtr.flush().expect("flushing CSV writer");
    Ok(())
}
