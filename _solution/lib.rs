use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn count_words(text: &str, ignored_words: &str) -> String {
    let text = text.to_lowercase();
    let ignored_words: Vec<_> = ignored_words.split("\n").collect();

    let mut word_count: Vec<(&str, i32)> = text
        .split_whitespace()
        .fold(HashMap::new(), |mut acc, word| {
            if ignored_words.contains(&word) {
                return acc;
            }
            let count = acc.entry(word).or_insert(0);
            *count += 1;
            acc
        })
        .into_iter()
        .collect();

    word_count.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    format!("{:?}", word_count.iter().take(5).collect::<Vec<_>>())
}
