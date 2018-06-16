extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let word_counter = build_word_counter("./small.txt");
    
    // println!("{:?}", word_counter);

    // println!("get value of a: {}", prob(&word_counter, "a"));

    // println!("get value of unknown {:?}", word_counter.get("fuck"));

    // println!("Known words: {:?}", known(vec!["haha", "a"], &word_counter));

    // println!("Edits of word: {:?}", edits("hello"));

    // println!("Edits of word: {:?}", edits_twice("hello"));

    println!("{}", correction("word", &word_counter));
}

fn correction(word: &str, counter: &HashMap<String, i32>) -> String {
    candidates(word, counter).iter().max_by_key(|x| 
        prob(counter, word)
    ).unwrap().to_string()
}

fn candidates(word: &str, counter: &HashMap<String, i32>) -> HashSet<String> {
    let mut origin: HashSet<String> = HashSet::new();
    origin.insert(word.to_string());
    match known(&origin, counter) {
        Some(words) => words,
        None => match known(&edits(word), counter) {
            Some(words) => words,
            None => match known(&edits_twice(word), counter) {
                Some(words) => words,
                None => origin,
            }
        }
    }
}

fn prob(counter: &HashMap<String, i32>, word: &str) -> f32 {
    match counter.get(word) {
        Some(num) => *num as f32 / counter.len() as f32,
        None => 0f32,
    }
}

fn known(words: &HashSet<String>, counter: &HashMap<String, i32>) -> Option<HashSet<String>> {
    let mut known_words: HashSet<String> = HashSet::new();
    for word in words {
        if counter.contains_key(word) {
            known_words.insert(word.to_string());
        }
    }
    if known_words.len() > 0 {
        Some(known_words)
    } else {
        None
    }
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

fn edits(word: &str) -> HashSet<String> {
    let letters = "abcdefghijklmnopqrstuvwxyz";
    let splits = (0..word.len() + 1)
        .map(|i| (&word[..i], &word[i..]))
        .collect::<Vec<_>>();

    let deletes = splits.iter()
        .filter(|(_, right)| right.len() > 0)
        .map(|(left, right)| left.to_string() + &right[1..].to_string())
        .collect::<Vec<_>>();

    let transposes = splits.iter()
        .filter(|(_, right)| right.len() > 1)
        .map(|(left, right)| left.to_string() + &right[1..2] + &right[0..1] + &right[2..])
        .collect::<Vec<_>>();

    let replaces = (0..letters.len()).flat_map(|i|
        splits.iter()
              .filter(|(_, right)| right.len() > 0)
              .map(|(left, right)| left.to_string() + &letters[i..i+1] + &right[1..])
              .collect::<Vec<_>>()
    ).collect::<Vec<_>>();

    let inserts = (0..letters.len()).flat_map(|i|
        splits.iter()
              .map(|(left, right)| left.to_string() + &letters[i..i+1] + &right.to_string())
              .collect::<Vec<_>>()
    ).collect::<Vec<_>>();

    let mut condidates: HashSet<String> = HashSet::new();
    for words in [deletes, transposes, replaces, inserts].iter() {
        for word in words {
            condidates.insert(word.to_string());
        }
    }
    condidates
}

fn edits_twice(word: &str) -> HashSet<String> {
    let mut candidates: HashSet<String> = HashSet::new();
    for edit_once in edits(word).iter() {
        for edit_twice in edits(edit_once).iter() {
            candidates.insert(edit_twice.to_string());
        }
    }
    candidates
}