use crate::{answer, common::Answer};
use std::collections::HashSet;

pub const ANSWER: Answer = answer!(157, 70);

fn prio(c: char) -> usize {
    match c {
        'a'..='z' => 1 + (c as u32 - 'a' as u32),
        'A'..='Z' => 27 + (c as u32 - 'A' as u32),
        _ => panic!("wrong char {}", c),
    }.try_into().unwrap()
}

fn process_rucksack(line: &String) -> usize {
    let hlen = line.len() / 2;
    let left: HashSet<char> = line[0..hlen].chars().collect();
    let right: HashSet<char> = line[hlen..].chars().collect();
    left.intersection(&right).map(|c| prio(*c)).sum()
}

fn process_triple(lines: &[String]) -> usize {
    let mut it = lines.iter();
    let k1: HashSet<char> = it.next().unwrap().chars().collect();
    let k2: HashSet<char> = it.next().unwrap().chars().collect();
    let k3: HashSet<char> = it.next().unwrap().chars().collect();
    let k12 : HashSet<char> = k1.intersection(&k2).copied().collect();
    k12.intersection(&k3).map(|c| prio(*c)).sum()
}

pub fn process_a(lines: Vec<String>) -> usize {
    lines.iter().map(process_rucksack).sum()
}

pub fn process_b(lines: Vec<String>) -> usize {
    lines.chunks(3).map(process_triple).sum()
}
