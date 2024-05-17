use std::path::PathBuf;
use structopt::StructOpt;

use super::errors::*;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(parse(from_os_str), help="The first of the 2 files you want to fuzzy-link")]
    pub file_a: PathBuf,

    #[structopt(parse(from_os_str), help="The second of the 2 files you want to fuzzy-link")]
    pub file_b: PathBuf,

    #[structopt(short="a", long, required=true, help="The column letters in File A that you want to search on, separated by a space (e.g. A C D F)")]
    pub file_a_cols: Vec<char>,

    #[structopt(short="b", long, required=true, help="The column letters in File B that you want to search on, separated by a space (e.g. G H I)")]
    pub file_b_cols: Vec<char>,

    #[structopt(short="o", long, required=true, help="The directory where you want the output to be saved")]
    pub output: PathBuf,

    #[structopt(short="t", long, required=true, help="The maximum difference allowed between entities, between 0.0 and 1.0")]
    pub tolerance: f32,
}

pub fn args() -> Result<Opt, Error> {
    let input = Opt::from_args();
    match input.tolerance <= 1.0 && input.tolerance >= 0.0 {
        true => Ok(input),
        false => Err(Error::ToleranceError { t: input.tolerance })
    }
}