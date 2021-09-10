use levenshtein::levenshtein;
use rayon::prelude::*;

mod col_indices;
mod create_tree;
mod user_input;
mod read_csv;

fn main() {
    let input = user_input::args();
    let (ents_a, vp) = create_tree::tree(&input, true);
    let ents_b: Vec<create_tree::Entity> = read_csv::read(&input, false);

    let matches: Vec<Option<&create_tree::Entity>> = ents_b.into_par_iter()
        .map(|ent| {
            let (index, _) = vp.find_nearest(&ent);
            if levenshtein(&ent.key, &ents_a[index].key) < input.tolerance { Some(&ents_a[index]) } else { None }
        })
        .collect();
    println!("{:#?}", matches.into_iter().filter(|m| match m {
        Some(_) => true,
        None => false
    }).collect::<Vec<Option<&create_tree::Entity>>>())
}
