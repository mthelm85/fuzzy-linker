use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid tolerance. {} is not between 0.0 and 1.0", t)]
    ToleranceError { t: f32 }
}