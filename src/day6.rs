use crate::{answer, common::Answer};
use itertools::Itertools;
use std::collections::HashMap;

pub const ANSWER: Answer = answer!("7", "19");

pub fn process_generic(lines: Vec<String>, window: usize) -> String {
    let line = &lines.first().unwrap();
    let chars : Vec<char> = line.chars().collect();
    let mut freq: HashMap<char, isize> = HashMap::new();
    for i in 0..chars.len() {
        if i >= window {
            freq.entry(chars[i-window]).and_modify(|c| *c -= 1);
        }
        freq.entry(chars[i]).and_modify(|c| *c += 1).or_insert(1);
        if freq.values().filter(|c| **c == 1).count() == window {
            return (i + 1).to_string();
        }
    }
    panic!("no way")
}

pub fn _process_a_v1(lines: Vec<String>) -> String {
    let first_idx: Option<usize> = lines
        .first()
        .unwrap()
        .chars()
        .enumerate()
        .tuple_windows::<(_, _, _, _)>()
        .filter(|( (_, a), (_, b), (_, c), (_, d))| 
             a != b && a != c && a != d && b != c && b != d && c != d
        )
        .map(|idx| idx.0.0 )
        .next();
    (first_idx.unwrap() + 4).to_string()
}

pub fn process_a(lines: Vec<String>) -> String {
    process_generic(lines, 4)
}

pub fn process_b(lines: Vec<String>) -> String {
    process_generic(lines, 14)
}
