use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let mut word_vec: Vec<_> = word_lower.chars().collect();
    word_vec.sort_unstable();
    possible_anagrams
        .iter()
        .filter(|anagram| {
            anagram.len() == word_lower.len() && *anagram.to_lowercase() != word_lower && {
                let mut anagram_vec: Vec<_> = anagram.to_lowercase().chars().collect();
                anagram_vec.sort_unstable();
                word_vec
                    .iter()
                    .zip(anagram_vec.iter())
                    .all(|(&a, &b)| a == b)
            }
        }).cloned()
//        .map(|x| *x)
        .collect::<HashSet<&str>>()
}
