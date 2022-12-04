use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!(2, 4);

type Interval = (usize, usize);

pub fn parse(line: &String) -> (Interval, Interval) {
    let components = line.split(&['-', ',']);
    let numbers: Vec<usize> = components.map(|s| s.parse::<usize>().unwrap()).collect();
    ((numbers[0], numbers[1]), (numbers[2], numbers[3]))
}

pub fn is_covered(a: Interval, b: Interval) -> bool {
    a.0 <= b.0 && a.1 >= b.1
}

pub fn process_a(lines: Vec<String>) -> usize {
    lines
        .iter()
        .map(parse)
        .filter(|(i1, i2)| is_covered(*i1, *i2) || is_covered(*i2, *i1))
        .count()
}

pub fn is_overlapped(a: Interval, b: Interval) -> bool {
    a.1 >= b.0 && a.0 <= b.1
}

pub fn process_b(lines: Vec<String>) -> usize {
    lines
        .iter()
        .map(parse)
        .filter(|(i1, i2)| is_overlapped(*i1, *i2))
        .count()
}
