extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let word_counter = build_word_counter("./small.txt");
    
    println!("{:?}", word_counter);

    println!("get value of a: {}", prob(&word_counter, "a"));

    println!("get value of unknown {:?}", word_counter.get("fuck"));

    println!("Known words: {:?}", known(vec!["haha", "a"], &word_counter));
}

fn prob(counter: &HashMap<String, i32>, word: &str) -> f32 {
    match counter.get(word) {
        Some(num) => *num as f32 / counter.len() as f32,
        None => 0f32,
    }
}

fn known(words: Vec<&str>, counter: &HashMap<String, i32>) -> HashSet<String> {
    let mut known_words: HashSet<String> = HashSet::new();
    for word in words {
        if counter.contains_key(word) {
            known_words.insert(word.to_string());
        }
    }
    known_words
}

fn build_word_counter(filepath: &str) -> HashMap<String, i32> {
    let mut file = File::open(filepath).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(r"\w+").unwrap();
    let mut word_counter: HashMap<String, i32> = HashMap::new();
    for cap in re.captures_iter(&contents) {
        let word = cap[0].to_lowercase();
        let counter = word_counter.entry(word).or_insert(0);
        *counter += 1;
    }

    word_counter
}
