#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
fn pattern_search<T: PartialEq>(pattern: &[T], text: &[T]) -> bool {
    if pattern.is_empty() {
        return true;
    }
    let pattern_len: usize = pattern.len();
    let mut p: Vec<i32> = vec![-1; pattern.len()];
    let mut i: i32 = -1;
    for j in 1..pattern_len {
        while i != -1 && pattern[(i + 1) as usize] != pattern[j] {
            i = p[i as usize];
        }
        p[j] = p[j - 1]
            + if pattern[(i + 1) as usize] == pattern[j] {
                1
            } else {
                0
            };
    }
    let mut i = -1;
    for t in text.iter() {
        while i != -1 && pattern[(i + 1) as usize] != *t {
            i = p[i as usize];
        }
        if pattern[(i + 1) as usize] == *t {
            i += 1;
            if i == (pattern_len - 1) as i32 {
                return true;
            }
        }
    }
    false
}
pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let (la, lb) = (_first_list.len(), _second_list.len());
    match (la, lb) {
        (x, y) if x == y => {
            if _first_list
                .iter()
                .zip(_second_list.iter())
                .all(|(a, b)| *a == *b)
            {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
        (x, y) if x > y => {
            if pattern_search(_second_list, _first_list) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        (x, y) if x < y => {
            if pattern_search(_first_list, _second_list) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        (_, _) => Comparison::Equal, // actually the code should never run to here
    }
}
