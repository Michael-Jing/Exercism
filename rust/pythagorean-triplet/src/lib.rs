use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (1_u32..sum / 3_u32)
        .into_par_iter()
        .filter_map(|a| {
            let b_plus_c: u32 = sum - a;
            let b: u32 = (b_plus_c.pow(2) - a.pow(2)) / (2 * b_plus_c);
            let c: u32 = b_plus_c - b;
            match a < b && (b_plus_c.pow(2) - a.pow(2)) % (2 * b_plus_c) == 0 {
                true => Some([a, b, c]),
                false => None,
            }
        })
        .collect::<HashSet<[u32; 3]>>()
}
