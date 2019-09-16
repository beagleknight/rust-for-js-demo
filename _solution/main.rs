use rust_for_js_demo::count_words;
use std::fs;

fn main() {
    let some_text = fs::read_to_string("file.txt").unwrap();
    let ignored_words = fs::read_to_string("stop_words.txt").unwrap();
    println!("{:?}", count_words(&some_text, &ignored_words));
}
