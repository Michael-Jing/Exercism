use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    (1..sum / 3)
        .map(|a| {
            let b_plus_c = sum - a;
            let b = (b_plus_c * b_plus_c - a * a) / 2 / b_plus_c;
            let c = b_plus_c - b;
            [a, b, c]
        })
        .filter(|v| {
            let [a, b, c] = v;
            a < b && b < c && a * a + b * b == c * c
        })
        .collect::<HashSet<[u32; 3]>>()
}
