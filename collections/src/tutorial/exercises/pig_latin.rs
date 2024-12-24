use std::collections::HashSet;
use once_cell::sync::Lazy;

static CONSONANTS: Lazy<HashSet<char>> = Lazy::new(||
    "qwrtypsdfghjklzxvbnmQWRTYPSDFGHJKLZXCVBNM"
    .chars()
    .collect()
);


pub fn split_and_convert(s: &String) -> String {
    s.split_whitespace()
        .map(|x| convert(&String::from(x)))
        .filter_map(|x| x)
        .collect::<Vec<_>>()
        .join(" ")
}


/**
 * In the case of an empty String, returns None
 */
pub fn convert(word: &String) -> Option<String> {
    match word.chars().nth(0) {
        Some(c) => {
            if consonant(&c) {
                Some(append_suffix(&word[1..], c))
            } else {
                Some(append_suffix(&word[..], 'h'))
            }
        },
        _ => None
    }
}

fn append_suffix(s: &str, c: char) -> String {
    format!("{s}-{c}ay")
}

fn consonant(c: &char) -> bool {
    CONSONANTS.contains(c)
}