use console::style;
use csv::Writer;
use indicatif::ParallelProgressIterator;
use levenshtein::levenshtein;
use rayon::prelude::*;
use serde::Serialize;

mod col_indices;
mod create_tree;
mod entity;
mod errors;
mod read_csv;
mod user_input;

#[derive(Debug, Serialize)]
struct CsvRecord {
    row_a: u32,
    row_b: u32,
}

fn main() {
    let input = match user_input::args() {
        Ok(input) => input,
        Err(e) => {
            eprintln!("{}", e);
            return
        }
    };

    println!(
        "{} Reading CSV files...",
        style("[1/4]").bold().dim()
    );

    let ents_a = match read_csv::read(&input, true) {
        Ok(ents) => ents,
        Err(e) => {
            eprintln!("{}", e);
            return
        }
    };

    let ents_b = match read_csv::read(&input, false) {
        Ok(ents) => ents,
        Err(e) => {
            eprintln!("{}", e);
            return
        }
    };
    
    println!(
        "{} Building VP tree...",
        style("[2/4]").bold().dim()
    );

    let vp = create_tree::tree(&ents_a);

    println!(
        "{} Searching for links...",
        style("[3/4]").bold().dim()
    );

    let end = ents_b.len() as u64;
    let matches: Vec<Option<CsvRecord>> = ents_b.into_par_iter()
        .progress_count(end)
        .map(|ent| {
            let (index, _) = vp.find_nearest(&ent);
            let max_len = std::cmp::max(ent.key.len(), ents_a[index].key.len());
            if levenshtein(&ent.key, &ents_a[index].key) < (input.tolerance * max_len as f32) as usize {
                Some(CsvRecord {
                    row_a: ents_a[index].i + 2,
                    row_b: ent.i + 2
                })
            } else { None }
        })
        .collect();
    let mut wtr = match Writer::from_path(&input.output) {
        Ok(wtr) => wtr,
        Err(_) => {
            eprintln!("{}", errors::Error::CsvWriterError { p: input.output });
            return
        }
    };

    println!(
        "{} Writing link (output) file...",
        style("[4/4]").bold().dim()
    );

    matches.iter()
        .for_each(|r| {
            if let Some(r) = r {
                match wtr.serialize(r) {
                    Ok(rcrd) => rcrd,
                    Err(e) => eprintln!("{}", e)
                }
            }
        });
    match wtr.flush() {
        Ok(_) => println!("{}", style("Done").green().bright()),
        Err(e) => eprintln!("{}", e)
    };
}
