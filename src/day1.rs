use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(24000, 45000);

pub fn process_a(lines: Vec<String>) -> usize {
    let mut elfs: Vec<usize> = vec![];
    let mut elf_calories: usize = 0;
    for line in lines {
        if line.is_empty() {
            elfs.push(elf_calories);
            elf_calories = 0;
        } else {
            elf_calories += line.parse::<usize>().unwrap();
        }
    }
    // last elf
    if elf_calories > 0 {
        elfs.push(elf_calories);
    }
    println!("{:?}", elfs);
    *elfs.iter().max().unwrap()
}

pub fn process_b(lines: Vec<String>) -> usize {
    let mut elfs: Vec<usize> = vec![];
    let mut elf_calories: usize = 0;
    for line in lines {
        if line.is_empty() {
            elfs.push(elf_calories);
            elf_calories = 0;
        } else {
            elf_calories += line.parse::<usize>().unwrap();
        }
    }
    // last elf
    if elf_calories > 0 {
        elfs.push(elf_calories);
    }
    elfs.sort_by(|a, b| b.cmp(a));
    elfs[0..3].iter().sum()
}
