use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!("15", "12");

#[derive(Eq, PartialEq)]
enum Figure {
    Rock,
    Paper,
    Scissors,
}

fn parse_figure(s: &str) -> Figure {
    // op: A for Rock, B for Paper, and C for Scissors
    // me: X for Rock, Y for Paper, and Z for Scissors
    match s {
        "A" | "X" => Figure::Rock,
        "B" | "Y" => Figure::Paper,
        "C" | "Z" => Figure::Scissors,
        _ => panic!("bad figure"),
    }
}

pub fn process_a(lines: Vec<String>) -> String {
    lines
        .iter()
        .map(|s| s.split_once(' ').unwrap())
        .map(|(op, me)| (parse_figure(op), parse_figure(me)))
        .map(|(op, me)| {
            let figure_result = match me {
                Figure::Rock => 1,
                Figure::Paper => 2,
                Figure::Scissors => 3,
            };
            let round_result = match (me, op) {
                // winning strategies:
                (Figure::Rock, Figure::Scissors) => 6,
                (Figure::Paper, Figure::Rock) => 6,
                (Figure::Scissors, Figure::Paper) => 6,
                // otherwise
                (x, y) => {
                    if x == y {
                        // draw
                        3
                    } else {
                        // lose
                        0
                    }
                }
            };
            round_result + figure_result
        })
        .sum::<usize>()
        .to_string()
}

pub fn process_b(lines: Vec<String>) -> String {
    lines
        .iter()
        .map(|s| s.split_once(' ').unwrap())
        .map(|(op, outcome)| (parse_figure(op), outcome))
        .map(|(op, outcome)| {
            let me = match (outcome, op) {
                // lose
                ("X", Figure::Rock) => Figure::Scissors,
                ("X", Figure::Paper) => Figure::Rock,
                ("X", Figure::Scissors) => Figure::Paper,
                // draw
                ("Y", op_figure) => op_figure,
                // win
                ("Z", Figure::Rock) => Figure::Paper,
                ("Z", Figure::Paper) => Figure::Scissors,
                ("Z", Figure::Scissors) => Figure::Rock,
                // that's it
                _ => panic!("bad input"),
            };
            let figure_result = match me {
                Figure::Rock => 1,
                Figure::Paper => 2,
                Figure::Scissors => 3,
            };
            let round_result = match outcome {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => panic!("bad input"),
            };
            round_result + figure_result
        })
        .sum::<usize>()
        .to_string()
}
