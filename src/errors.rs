use std::path::PathBuf;

use console::style;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{} Invalid tolerance. {} is not between 0.0 and 1.0", style("Error:").red().bright(), style(t).red().bright())]
    ToleranceError { t: f32 },
    #[error("{} Invalid column. {} is not in the set {{A,...,Z}}", style("Error:").red().bright(), style(c).red().bright())]
    ColumnError { c: char },
    #[error("{} Unable to create CSV reader from path {:?}", style("Error:").red().bright(), style(p).red().bright())]
    CsvReaderError { p: PathBuf },
    #[error("{} Unable to create CSV writer from path {:?}", style("Error:").red().bright(), style(p).red().bright())]
    CsvWriterError { p: PathBuf },
    #[error("{} Unable to parse record, skipping...", style("Error:").red().bright())]
    CsvParseError
}