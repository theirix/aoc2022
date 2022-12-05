use crate::{answer, common::Answer};

pub const ANSWER: Answer = answer!("CMZ", "MCD");

type Stacks = Vec<Vec<char>>;
type Commands = Vec<(usize, usize, usize)>;

fn parse_input(lines: Vec<String>) -> (Stacks, Commands) {
    let mut stacks: Stacks = vec![];
    let mut commands: Commands = vec![];

    for line in lines {
        if line.contains('[') {
            // Parse stack in each line
            // indices: 1+0, 1+4, 1+8 ... 1+k*4
            let mut ind = 0;
            while let Some(c) = line.chars().nth(1 + ind * 4) {
                if stacks.len() <= ind {
                    stacks.push(vec![]);
                }
                if c != ' ' {
                    stacks[ind].push(c);
                }
                ind += 1;
            }
        }
        if line.starts_with("move") {
            // Parse commands to a triple
            let mut components = line.split(" ");
            commands.push((
                components.nth(1).unwrap().parse().unwrap(),
                (components.nth(1).unwrap().parse::<isize>().unwrap() - 1) as usize,
                (components.nth(1).unwrap().parse::<isize>().unwrap() - 1) as usize,
            ));
        }
    }
    for st in &mut stacks {
        st.reverse();
    }

    (stacks, commands)
}

pub fn process_a(lines: Vec<String>) -> String {
    let (mut stacks, commands) = parse_input(lines);

    // Execute commands
    for command in commands {
        let (mut amount, from, to) = command;
        while amount > 0 {
            let val = stacks[from].pop().expect("No more cargos");
            stacks[to].push(val);
            amount -= 1;
        }
    }
    stacks.iter_mut().map(|st| st.pop().unwrap()).collect()
}

pub fn process_b(lines: Vec<String>) -> String {
    let (mut stacks, commands) = parse_input(lines);

    // Execute commands
    for command in commands {
        let (amount, from, to) = command;
        let from_len = stacks[from].len();
        let vals = stacks[from].split_off(from_len - amount);
        stacks[to].extend(vals);
    }
    stacks.iter_mut().map(|st| st.pop().unwrap()).collect()
}
