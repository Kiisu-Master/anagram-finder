use dirs::home_dir;
use rayon::prelude::*;
use std::env::args;
use std::str::from_utf8;
use std::time::Instant;
use std::{fs, io};

fn main() {
    let args: Vec<_> = args().collect();
    let mut path = home_dir().unwrap().to_str().unwrap().to_owned();
    // First arg is language like de or en.
    if args.len() >= 2 {
        path += format!("\\.anagram-dictionary-{}.txt", args[1]).as_ref();
    } else {
        println!("Missing language arg");
        return;
    }
    let file = fs::read(path).unwrap();
    let candidates: Vec<&str> = from_utf8(&file).unwrap().split("\n").collect();
    let candidates: &[&str] = candidates.as_ref();

    // Handle only command line argument if passed.
    if args.len() == 3 {
        let target: &str = args[2].as_ref();
        let before = Instant::now();
        println!("Found anagrams: {:?}", target.get_anagrams(candidates));
        println!("Search duration: {}ms", before.elapsed().as_millis());
        return;
    }
    println!("Total word count: {}", candidates.len());
    
    // Enter anagram finding input loop.
    loop {
        println!("Enter target word:");
        let mut target = String::new();
        io::stdin()
            .read_line(&mut target)
            .expect("Failed to read line");
        let target = target.trim();
        let before = Instant::now();
        println!("Found anagrams: {:?}", target.get_anagrams(candidates));
        println!("Search duration: {}ms", before.elapsed().as_millis());
    }
}

trait Anagram {
    fn get_anagrams<'a>(&'a self, candidates: &'a [&'a str]) -> Vec<&&str>;
    fn is_anagram_of(&self, word: &str) -> bool;
}

// Implement Anagram for both &str and String.
impl<T: AsRef<str>> Anagram for T {
    #[inline]
    fn get_anagrams<'a>(&'a self, candidates: &'a [&'a str]) -> Vec<&&str> {
        let target = self.as_ref();
        let target_lower = target.to_lowercase();
        candidates
            .par_iter()
            .filter(|candidate| candidate.len() == target.len())
            .filter(|candidate| **candidate != target)
            .filter(|candidate| candidate.to_lowercase().is_anagram_of(&target_lower))
            .collect()
    }
    #[inline]
    fn is_anagram_of(&self, candidate: &str) -> bool {
        let target = self.as_ref();
        let mut candidate = candidate.to_owned();
        for c in target.chars() {
            match candidate.find(c) {
                Some(i) => {
                    candidate.remove(i);
                }
                None => {
                    return false;
                }
            }
        }
        true
    }
}
