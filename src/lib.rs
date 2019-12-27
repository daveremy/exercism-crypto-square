pub fn encrypt(input: &str) -> String {
    let (num_rows, num_cols, normalized) = normalize(input);
    (0..num_cols)
        .map(|j| {
            (0..num_rows)
                .map(|i| normalized[(i * num_cols) + j])
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>()
        .iter()
        .map(|r| r.iter().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

/// First, the input is normalized: the spaces and punctuation are removed from the English
/// text and the message is downcased. Then, the normalized characters are broken into rows.
/// These rows can be regarded as forming a rectangle when printed with intervening newlines.
fn normalize(input: &str) -> (usize, usize, Vec<char>) {
    let mut normalized: Vec<char> = input
        .chars()
        .filter(|c| !c.is_ascii_punctuation() && !c.is_ascii_whitespace())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let (num_rows, num_cols) = num_rows_cols(normalized.len());
    pad(&mut normalized, num_rows, num_cols);
    (num_rows, num_cols, normalized)
}

/// Determine number of rows and columns: such that c >= r and c - r <= 1, where c is the
/// number of columns and r is the number of rows.
fn num_rows_cols(len: usize) -> (usize, usize) {
    let sqrt = (len as f64).sqrt();
    let r = sqrt.trunc() as usize;
    let c = if sqrt.fract() == 0.0 { r } else { r + 1 };
    (r, c)
}

/// Pad the end of the normalized so that the text will fully represent a rectangle.
fn pad(normalized: &mut Vec<char>, num_rows: usize, num_cols: usize) -> () {
    let num_spaces_to_add = (num_rows * num_cols) - normalized.len();
    normalized.append(&mut (0..num_spaces_to_add).map(|_| ' ').collect::<Vec<char>>());
}
