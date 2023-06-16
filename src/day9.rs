use crate::{answer, common::Answer};
use std::collections::HashSet;

pub const ANSWER: Answer = answer!("13", "1");
// b day9b -> 36

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

type Rope = Vec<Pos>;

#[derive(Debug, Clone)]
enum Move {
    R(i32),
    L(i32),
    U(i32),
    D(i32),
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        let (cop, cval) = s.split_once(' ').unwrap();
        let val: i32 = cval.parse().unwrap();
        match cop {
            "R" => Move::R(val),
            "L" => Move::L(val),
            "U" => Move::U(val),
            "D" => Move::D(val),
            _ => panic!("bad input {}", cop),
        }
    }
}

fn handle(mov: Move, h: &mut Pos, rope: &mut Rope, visited: &mut HashSet<Pos>) {
    catchup_rope(h, rope);
    show(h, rope, 6, 5);
    visited.insert(*rope.last().unwrap());
    match mov {
        Move::R(val) if val > 0 => {
            h.x += 1;
            handle(Move::R(val - 1), h, rope, visited);
        }
        Move::L(val) if val > 0 => {
            h.x -= 1;
            handle(Move::L(val - 1), h, rope, visited);
        }
        Move::U(val) if val > 0 => {
            h.y += 1;
            handle(Move::U(val - 1), h, rope, visited);
        }
        Move::D(val) if val > 0 => {
            h.y -= 1;
            handle(Move::D(val - 1), h, rope, visited);
        }
        _ => {}
    }
}

fn catchup(h: &Pos, t: &mut Pos) {
    let dx = h.x - t.x;
    let dy = h.y - t.y;
    if dx != 0 && dy != 0 && (dx.abs() > 1 || dy.abs() > 1) {
        // diagonally
        t.x += if dx > 0 { 1 } else { -1 };
        t.y += if dy > 0 { 1 } else { -1 };
    } else if dx.abs() > 1 {
        // horizontally
        t.x += if dx > 0 { 1 } else { -1 };
    } else if dy.abs() > 1 {
        // vertically
        t.y += if dy > 0 { 1 } else { -1 };
    }
}

fn catchup_rope(h: &Pos, rope: &mut Rope) {
    catchup(h, &mut rope[0]);
    for idx in 0..rope.len() - 1 {
        let mut upd = rope[idx + 1];
        catchup(&rope[idx], &mut upd);
        rope[idx + 1] = upd;
    }
}

fn show(h: &Pos, rope: &Rope, width: i32, height: i32) {
    for y in (0..height).rev() {
        for x in 0..width {
            let pos = Pos { x, y };
            let mut c = '.';
            for (idx, t) in rope.iter().enumerate().rev() {
                if pos == *t {
                    c = (idx + 1).to_string().chars().last().unwrap();
                }
            }
            if pos == *h {
                c = 'H';
            }
            print!("{}", c);
        }
        println!();
    }
}

pub fn process_generic(lines: Vec<String>, rope_len: usize) -> String {
    let moves: Vec<Move> = lines.iter().map(|s| s.as_str().into()).collect();

    let mut h = Pos { x: 0, y: 0 };
    let mut rope: Rope = vec![h; rope_len];
    let mut visited = HashSet::<Pos>::new();
    for mov in moves {
        handle(mov, &mut h, &mut rope, &mut visited);
    }
    let result = visited.len();
    result.to_string()
}

pub fn process_a(lines: Vec<String>) -> String {
    process_generic(lines, 1)
}

pub fn process_b(lines: Vec<String>) -> String {
    process_generic(lines, 9)
}
