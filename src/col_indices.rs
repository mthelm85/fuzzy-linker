use super::errors::*;

const COLUMN_LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 
    'F', 'G', 'H', 'I', 'J', 
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 
    'U', 'V', 'W', 'X', 'Y', 
    'Z',
];

pub fn cols(cols: &[char]) -> Result<Vec<usize>, Error> {
    cols.iter().map(|&c| {
        COLUMN_LETTERS.iter()
            .position(|&l| l == c)
            .ok_or(Error::ColumnError { c })
    }).collect()
}