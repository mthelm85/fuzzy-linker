const COLUMN_LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 
    'F', 'F', 'H', 'I', 'J', 
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 
    'U', 'V', 'W', 'X', 'Y', 
    'Z',
];

pub fn cols(cols: &[char]) -> Vec<usize> {
    cols.iter().map(|&c| {
        COLUMN_LETTERS.iter()
            .position(|&l| l == c)
            .unwrap_or_else(|| panic!("Unable to get column index. {} is not in the range A - Z", c))
    }).collect()
}