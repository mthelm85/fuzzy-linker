use std::process;

use csv::Reader;

use super::col_indices::*;
use super::entity::*;
use super::errors::*;
use super::user_input::*;

pub fn read(input: &Opt, a: bool) -> Result<Vec<Entity>, Error> {
    let mut rdr = match a {
        true => match Reader::from_path(&input.file_a) {
            Ok(rdr) => rdr,
            Err(_) => {
                eprintln!("{}", Error::CsvReaderError { p: input.file_a.clone() });
                process::exit(1)
            }
        },
        false => match Reader::from_path(&input.file_b) {
            Ok(rdr) => rdr,
            Err(_) => {
                eprintln!("{}", Error::CsvReaderError { p: input.file_b.clone() });
                process::exit(1)
            }
        }
    };

    let indices = match a {
        true => cols(&input.file_a_cols)?,
        false => cols(&input.file_b_cols)?
    };

    let mut ents: Vec<Entity> = Vec::new();

    for (row, result) in rdr.byte_records().enumerate() {
        let record = match result {
            Ok(rcrd) => rcrd,
            Err(_) => {
                eprintln!("{}", Error::CsvParseError);
                continue
            }
        };
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

    Ok(ents)
}