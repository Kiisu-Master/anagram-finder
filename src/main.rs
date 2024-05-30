use std::time::Instant;
use random_word::{self, all, Lang};
use std::env::args;

fn main() {
    let lang = Lang::En;
    let args: Vec<_> = args().collect();
    if !args.len() == 1 {
        return;
    }
    let target: &str = args[1].as_ref();
    let candidates = all(lang);
    let before = Instant::now();
    println!("Found anagrams {:?}", target.get_anagrams(candidates));
    println!("Search duration {}ms", before.elapsed().as_millis());
    println!("Checked words {}", candidates.len());
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
        let target = self.as_ref().to_lowercase();
        candidates
            .iter()
            .filter(|candidate| candidate.to_lowercase().is_anagram_of(&target))
            .collect()
    }
    #[inline]
    fn is_anagram_of(&self, candidate: &str) -> bool {
        let target = self.as_ref();
        if target == candidate || target.len() != candidate.len() {
            return false;
        }
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
