use rayon::prelude::*;
use std::env::args;
use std::str::from_utf8;
use std::time::Instant;
use std::{fs, io};
use dirs::home_dir;

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
    if args.len() == 3 {
        // Handle command line argument only if passed.
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
    // let run_count = 100;
    // let mut my_dur = 0 as f64;
    // let mut ref_dur = 0 as f64;
    // for _i in 1..=run_count {
    //     let my_before = Instant::now();
    //     let _my_result: Vec<&&str> = target.get_anagrams(&candidates);
    //     my_dur += my_before.elapsed().as_nanos() as f64;
    // }
    // for _i in 1..=run_count {
    //     let ref_before = Instant::now();
    //     let _reference_result = anagrams_for(target, &candidates);
    //     ref_dur += ref_before.elapsed().as_nanos() as f64;
    // }
    // let reference_result = anagrams_for(target, &candidates);
    // let my_result: Vec<&&str> = target.get_anagrams(&candidates);
    // for anagram in reference_result {
    //     if !my_result.iter().any(|x| *x.to_owned() == anagram) {
    //         println!("Result doesn't match");
    //         break;
    //     }
    // }
    // println!("Run count {}", run_count);
    // println!("My impl took {} nanoseconds", my_dur / run_count as f64);
    // println!("Ref impl took {} nanoseconds", ref_dur / run_count as f64);
    // println!("Difference {}", ref_dur / my_dur);
}

trait Anagram {
    fn get_anagrams<'a>(&'a self, candidates: &'a [&'a str]) -> Vec<&&str>;
    fn is_anagram_of(&self, word: &str) -> bool;
}

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

// use std::collections::HashSet;

// #[inline]
// fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
//     let word_lower = word.to_lowercase();
//     let word_sorted = get_sorted(&word_lower);
//     possible_anagrams
//         .iter()
//         .filter(|candidate| {
//             let candidate_lower = candidate.to_lowercase();
//             candidate_lower.len() == word_lower.len()
//                 && candidate_lower != word_lower
//                 && get_sorted(&candidate_lower) == word_sorted
//         })
//         .copied()
//         .collect()
// }

// #[inline]
// fn get_sorted(word: &str) -> Vec<char> {
//     let mut word_sorted: Vec<char> = word.chars().collect();
//     word_sorted.sort_unstable();
//     word_sorted
// }
