use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut set = HashSet::new();
    let word_lower = word.to_lowercase();
    let mut word_vec: Vec<_> = word_lower.chars().collect();
    word_vec.sort_unstable();
    for anagram in possible_anagrams {
        if anagram.len() == word_lower.len() && *anagram.to_lowercase() != word_lower {
            let mut anagram_vec: Vec<_> = anagram.to_lowercase().chars().collect();
            anagram_vec.sort_unstable();
            if word_vec.iter().eq(anagram_vec.iter()) {
                set.insert(*anagram);
            }
        }
    }
    set
}
