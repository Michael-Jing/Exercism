use std::char;
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return vec![];
    }
    let n_row = minefield.len() as i32;
    let n_col = minefield[0].len() as i32;
    minefield
        .iter()
        .enumerate()
        .map(|(r, s)| {
            s.chars()
                .enumerate()
                .map(|(col, c)| match c as char {
                    '*' => '*',
                    ' ' => {
                        let count: usize = [
                            (-1_i32, 0),
                            (-1, 1),
                            (0, 1),
                            (1, 1),
                            (1, 0),
                            (1, -1_i32),
                            (0, -1_i32),
                            (-1_i32, -1_i32),
                        ]
                        .iter()
                        .map(|(dr, dcol)| {
                            let (nr, ncol) = (r as i32 + dr, col as i32 + dcol);
                            if 0 <= nr
                                && nr < n_row
                                && 0 <= ncol
                                && ncol < n_col
                                && minefield[nr as usize].chars().nth(ncol as usize) == Some('*')
                            {
                                1
                            } else {
                                0
                            }
                        })
                        .sum();
                        if count > 0 {
                            char::from_digit(count as u32, 10).unwrap()
                        } else {
                            ' '
                        }
                    }
                    _ => ' ',
                })
                .collect()
        })
        .collect::<Vec<String>>()
}
