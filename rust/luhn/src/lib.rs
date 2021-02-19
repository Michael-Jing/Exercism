/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code
        .chars()
        .any(|c| !(c.is_ascii_whitespace() || c.is_ascii_digit()))
    {
        return false;
    }
    let (length, tot): (usize, u32) = code
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .rev()
        .enumerate()
        .fold((0, 0), |(count, tot), (i, c)| match i & 1 {
            0 => (count + 1, tot + c.to_digit(10).unwrap()),
            1 => (
                count + 1,
                tot + {
                    let n = 2 * c.to_digit(10).unwrap();
                    if n > 9 {
                        n - 9
                    } else {
                        n
                    }
                },
            ),
            _ => (count, tot),
        });
    !(length <= 1 || tot % 10 != 0)
}
