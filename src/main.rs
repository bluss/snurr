
extern crate rand;
extern crate itertools;
extern crate fixedbitset;

use itertools::Itertools;
use itertools::Partition;
use fixedbitset::FixedBitSet;

use std::time::Instant;
use std::path::Path;
use std::io::prelude::*;
use std::io::Error;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::iter::FromIterator;

use rand::{Rng};

const SUBANAGRAM_MIN_LENGTH: usize = 3;
const TARGET_WORD_LENGTH_MIN: usize = 7;
const TARGET_WORD_LENGTH_MAX: usize = 8;
const WORD_TIME_LIMIT_SECS: u64 = 60;

fn readfile(p: &Path) -> Result<Vec<String>, Error> {
    let f = try!(File::open(p));
    BufReader::new(f).lines().collect::<Result<Vec<_>, _>>()
}

#[derive(Clone, Debug)]
pub struct Word {
    word: String,
    char_len: usize,
    chars: Vec<(char, u32)>,
}

impl Word {
    pub fn new(word: &str) -> Self {
        // sort and count the characters in the big word
        let c = word.chars().count();
        let mut chars: Vec<(char, u32)> = Vec::new();
        for ch in word.chars() {
            match chars.binary_search_by(|t| t.0.cmp(&ch)) {
                Ok(i) => chars[i].1 += 1,
                Err(i) => { chars.insert(i, (ch, 1)); }
            }
        }
        Word {
            word: word.to_string(),
            char_len: c,
            chars: chars,
        }
    }

    // if `s` is a subanangram
    pub fn contains_subanagram(&self, small: &str) -> bool {
        let sn = small.chars().count();
        if sn > self.char_len { return false; }

        let mut chars = self.chars.clone();
        for ch in small.chars() {
            match chars.binary_search_by(|t| t.0.cmp(&ch)) {
                Ok(i) => {
                    if chars[i].1 == 0 {
                        return false;
                    }
                    chars[i].1 -= 1;
                }
                Err(_) => return false,
            }
        }
        true
    }

    // if `small` has the same letters, without counting occurances
    pub fn has_same_letters(&self, small: &str) -> bool {
        for ch in small.chars() {
            match self.chars.binary_search_by(|t| t.0.cmp(&ch)) {
                Ok(_) => { }
                Err(_) => return false,
            }
        }
        true
    }
}

fn main() {
    let args = env::args();
    if args.len() <= 1 {
        println!("Error: Need a word list as argument");
        std::process::exit(1);
    }
    let mut list = Vec::new();
    for arg in args.skip(1) {
        let mut list_ = readfile(Path::new(&arg)).unwrap();
        list.append(&mut list_);
    }

    // remove everything that's not lowercase (removes accents and hyphens too)
    list.retain(|elt| {
        elt.chars().all(|c| c.is_lowercase())
    });


    let mut rng = rand::weak_rng();
    let long_word = rand::sample(&mut rng, list.iter().filter(|s| {
        let count = s.chars().count();
        count >= TARGET_WORD_LENGTH_MIN && count <= TARGET_WORD_LENGTH_MAX
    }), 1);
    let target_word = long_word.get(0).expect("No word of the right length in wordlist!").clone();
    let word_info = Word::new(target_word);

    let mut words = Vec::new();
    for word in &list {
        if word.chars().count() >= SUBANAGRAM_MIN_LENGTH && word_info.contains_subanagram(&word) {
            words.push(word);
            //println!("subanagram: {} is in {}", word, target_word);
        }
    }
    words.sort();
    println!("");
    println!("The word has {} letters and it forms {} words",
             target_word.chars().count(), words.len());


    let mut target_shuffled = Vec::from_iter(target_word.chars());
    rng.shuffle(&mut target_shuffled);

    let mut found_words = FixedBitSet::with_capacity(words.len());
    let mut points = 0;
    let start_time = Instant::now();

    println!("{}    you have {} points and {} seconds left",
             target_shuffled.iter().format_default(" "),
             points,
             WORD_TIME_LIMIT_SECS
            );


    let stdio = std::io::stdin();
    let mut input_line = String::new();
    loop {
        input_line.clear();
        if let Err(e) = stdio.read_line(&mut input_line) {
            println!("Error: {}", e);
            std::process::exit(1);
        }
        if input_line.is_empty() {
            break;
        }

        let elapsed = start_time.elapsed().as_secs();
        if elapsed > WORD_TIME_LIMIT_SECS {
            println!("Time's up!");
            break;
        }

        let input_word = input_line.trim();

        if input_word.is_empty() {
            rng.shuffle(&mut target_shuffled);
            println!("{}    you have {} points and {} seconds left",
                     target_shuffled.iter().format_default(" "),
                     points,
                     WORD_TIME_LIMIT_SECS - elapsed,
                    );
            continue;
        }
        let lowercase_input = String::from_iter(input_word.chars().flat_map(|c| c.to_lowercase()));

        if let Ok(i) = words.binary_search(&&lowercase_input) {
            let has = found_words.contains(i);
            found_words.insert(i);
            if !has && lowercase_input == *target_word {
                points += 5;
                println!("+5 points for >>){}(<< !", words[i]);
            } else if !has {
                points += 1;
                println!("+1 points for {}", words[i]);
            } else {
                println!("{} has already been found.", words[i]);
            }
        } else {
            //println!("No points for input {}", lowercase_input);
        }
    }
    let (found, not_found): (Vec<_>, Vec<_>) = words.iter().enumerate().partition_map(|(i, s)| {
        if found_words.contains(i) {
            Partition::Left(*s)
        } else {
            Partition::Right(*s)
        }
    });
    println!("Finished with {} points.", points);
    println!("Target word: {}", target_word);
    println!("Found words: {}", found.iter().format_default(", "));
    println!("Missed: {}", not_found.iter().format_default(", "));
}
