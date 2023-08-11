use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!("13140", "EYES");

enum Command {
    Noop,
    Addx(isize),
}

type Commands = Vec<Command>;

fn parse_line(line: &String) -> Command {
    if line == "noop" {
        Command::Noop
    } else if line.starts_with("addx") {
        let mut components = line.split(' ');
        let value: isize = components.nth(1).unwrap().parse().unwrap();
        Command::Addx(value)
    } else {
        panic!("Wrong command {}", line)
    }
}

fn parse_input(lines: Vec<String>) -> Commands {
    lines.iter().map(parse_line).collect()
}

pub fn process_a(lines: Vec<String>) -> String {
    let commands = parse_input(lines);

    let points = vec![20, 60, 100, 140, 180, 220];

    let mut result: isize = 0;
    let mut x: isize = 1;
    let mut addx_value: Option<isize> = None;
    let mut command_iter = commands.iter();
    for cycle in 1.. {
        if points.contains(&cycle) {
            let strength = (cycle as isize) * x;
            println!("=> cycle={} x={} strength={}", cycle, x, strength);
            result += strength;
        }
        if let Some(value) = addx_value {
            x += value;
            addx_value = None;
        } else if let Some(command) = command_iter.next() {
            if let Command::Addx(value) = command {
                addx_value = Some(*value)
            }
        } else {
            break;
        }
    }

    result.to_string()
}

pub fn process_b(lines: Vec<String>) -> String {
    let commands = parse_input(lines);

    const ROWS: usize = 6;
    const COLS: usize = 40;
    let mut screen = vec![[false; COLS]; ROWS];

    let mut x: isize = 1;
    let mut addx_value: Option<isize> = None;
    let mut command_iter = commands.iter();
    for cycle in 0usize.. {
        let row: usize = cycle / COLS;
        let col: usize = cycle % COLS;
        if (col as isize - x).abs() <= 1 {
            screen[row][col] = true;
        }
        if let Some(value) = addx_value {
            x += value;
            addx_value = None;
        } else if let Some(command) = command_iter.next() {
            if let Command::Addx(value) = command {
                addx_value = Some(*value)
            }
        } else {
            break;
        }
    }

    for row in 0..ROWS {
        for col in 0..COLS {
            print!("{}", if screen[row][col] { '#' } else { ' ' });
        }
        println!();
    }

    "".into()
}
