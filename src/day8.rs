use crate::{answer, common::Answer};
use colored::*;
use std::collections::HashSet;

pub const ANSWER: Answer = answer!("21", "8");

type Pos = (isize, isize);

pub fn process_a(lines: Vec<String>) -> String {
    let grid: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let rows = grid.len() as isize;
    let cols = grid.first().unwrap().len() as isize;

    let mut visible = HashSet::<Pos>::new();

    for r in 0..rows {
        visible.insert((r, 0));
        visible.insert((r, cols - 1));
    }
    for c in 0..cols {
        visible.insert((0, c));
        visible.insert((rows - 1, c));
    }

    for r in 0..rows {
        let mut highest = u32::min_value();
        for c in 0..cols {
            let h = grid[r as usize][c as usize];
            if h > highest {
                visible.insert((r, c));
            }
            highest = highest.max(h);
        }
        highest = u32::min_value();
        for c in (0..cols).rev() {
            let h = grid[r as usize][c as usize];
            if h > highest {
                visible.insert((r, c));
            }
            highest = highest.max(h);
        }
    }

    for c in 0..cols {
        let mut highest = u32::min_value();
        for r in 0..rows {
            let h = grid[r as usize][c as usize];
            if h > highest {
                visible.insert((r, c));
            }
            highest = highest.max(h);
        }
        highest = u32::min_value();
        for r in (0..rows).rev() {
            let h = grid[r as usize][c as usize];
            if h > highest {
                visible.insert((r, c));
            }
            highest = highest.max(h);
        }
    }

    // Pretty print tree
    for r in 0..rows {
        for c in 0..cols {
            if visible.contains(&(r, c)) {
                print!("{}", grid[r as usize][c as usize].to_string().green());
            } else {
                print!("{}", grid[r as usize][c as usize].to_string().red());
            }
        }
        println!();
    }

    visible.len().to_string()
}

pub fn process_b(lines: Vec<String>) -> String {
    let grid: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let rows = grid.len() as isize;
    let cols = grid.first().unwrap().len() as isize;

    let mut max_score: usize = 0;
    for r0 in 1..rows - 1 {
        for c0 in 1..cols - 1 {
            let h = grid[r0 as usize][c0 as usize];
            let mut scores = [0; 4];
            for r in (0..r0).rev() {
                scores[0] += 1;
                if grid[r as usize][c0 as usize] >= h {
                    break;
                }
            }
            for c in (0..c0).rev() {
                scores[1] += 1;
                if grid[r0 as usize][c as usize] >= h {
                    break;
                }
            }
            for c in c0 + 1..cols {
                scores[2] += 1;
                if grid[r0 as usize][c as usize] >= h {
                    break;
                }
            }
            for r in r0 + 1..rows {
                scores[3] += 1;
                if grid[r as usize][c0 as usize] >= h {
                    break;
                }
            }
            let cur_score = scores.iter().product();
            max_score = max_score.max(cur_score);
        }
    }

    max_score.to_string()
}
