extern crate regex;
extern crate hashbrown;

use hashbrown::HashMap;

use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::error;
use std::time::Instant;


const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Debug)]
struct Dictionary {
    counter: HashMap<String, u32>,
}

impl Dictionary {
    fn new(filepath: &str) -> Result<Dictionary, Box<error::Error>> {
        let mut f = File::open(filepath)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        let re = Regex::new(r"\w+")?;
        let mut counter: HashMap<String, u32> = HashMap::new();
        for cap in re.captures_iter(&contents) {
            let word = cap[0].to_lowercase();
            *counter.entry(word).or_insert(0) += 1;
        }

        Ok(Dictionary {counter: counter})
    }

    fn capacity(&self) -> usize {
        self.counter.len()
    }

    fn known(&self, words: &mut Vec<String>) -> usize {
        words.retain(|word| self.counter.contains_key(word));
        words.len()
    }

    fn correct(&self, word: &str) -> String {
        self.candidates(word).iter().max_by(|&x, &y| {
            let count_x = self.counter.get(x);
            let count_y = self.counter.get(y);
            count_x.cmp(&count_y)
        }).unwrap().to_string()
    }

    fn candidates(&self, word: &str) -> Vec<String> {
        let cands: Vec<String> = vec![word.to_string()];
        if self.known(&mut cands.clone()) > 0 {
            cands
        } else {
            let mut edition = edit_once(word);
            if self.known(&mut edition) > 0 {
                edition
            } else {
                let mut edition = edit_twice(word);
                if self.known(&mut edition) > 0 {
                    edition
                } else {
                    cands
                }
            }
        }
    }
}

fn edit_once(word: &str) -> Vec<String> {
    let splits = (0..=word.len())
        .map(|i| (&word[..i], &word[i..]));

    let deletes = splits.clone()
        .filter(|(_, right)| right.len() > 0)
        .map(|(left, right)| [left, &right[1..]].concat());

    let transposes = splits.clone()
        .filter(|(_, right)| right.len() > 1)
        .map(|(left, right)| [left, &right[1..2], &right[0..1], &right[2..]].concat());

    let replaces = (0..26).flat_map(|i|
        splits.clone()
            .filter(|(_, right)| right.len() > 0)
            .map(move |(left, right)| [left, &ALPHABET[i..i+1], &right[1..]].concat())
        );

    let splits_clone = splits.clone();
    let inserts = (0..26).flat_map(move |i|
        splits_clone.clone()
            .map(move |(left, right)| [left, &ALPHABET[i..i+1], right].concat())
        );

    let mut candidates = vec![];
    candidates.extend(deletes);
    candidates.extend(transposes);
    candidates.extend(replaces);
    candidates.extend(inserts);
    candidates
}

fn edit_twice(word: &str) -> Vec<String> {
    edit_once(word).into_iter().flat_map(|once| edit_once(&once)).collect()
}

fn main() {
    let dic = Dictionary::new("./sherlock.txt").unwrap();
    println!("Capacity: {}", dic.capacity());

    for word in vec!["helle", "world", "pythn", "nica", "dictionere"] {
        let start = Instant::now();
        println!("{} is corrected to {}", word, dic.correct(word));
        let duration = start.elapsed();
        println!("Finished in {:?}", duration);
    }
}
