use crate::{answer, common::Answer};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

pub const ANSWER: Answer = answer!("95437", "24933642");

enum Node {
    Dir {
        name: String,
        childs: Vec<Rc<RefCell<Node>>>,
    },
    File {
        name: String,
        size: usize,
    },
}

fn display(node: Rc<RefCell<Node>>, depth: usize, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    for _ in 0..depth {
        write!(fmt, "  ")?;
    }
    match &*node.borrow() {
        Node::Dir {
            ref name,
            ref childs,
        } => {
            writeln!(fmt, "dir {}", name,)?;
            for c in childs {
                display(c.clone(), depth + 1, fmt)?;
            }
        }
        Node::File { name, size } => writeln!(fmt, "file {} of {}", name, size)?,
    }
    Ok(())
}

impl fmt::Display for Node {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Node::Dir { childs, .. } = self {
            for c in childs {
                display(c.clone(), 1, fmt)?;
            }
        };
        Ok(())
    }
}

fn build_tree(lines: Vec<String>) -> Rc<RefCell<Node>> {
    let mut stack: Vec<Rc<RefCell<Node>>> = vec![];
    let mut idx = 0;
    while idx < lines.len() {
        let line = &lines[idx];
        println!("Command {}", &line);

        if line == "$ cd .." {
            stack.pop().unwrap();
            idx += 1
        } else if line.starts_with("$ cd") && stack.is_empty() {
            let new_root: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node::Dir {
                name: "/".into(),
                childs: vec![],
            }));
            stack.push(new_root);
        } else if line.starts_with("$ cd") {
            let subdir_str = &line[5..];
            println!("Cd to {}", subdir_str);
            idx += 1;

            let cur = stack.last_mut().unwrap();

            let mut new_cur: Option<Rc<RefCell<Node>>> = None;

            // try to find existing subdirectory
            if let Node::Dir { ref childs, .. } = *cur.borrow() {
                for n in childs {
                    if let Node::Dir { ref name, .. } = *n.borrow_mut() {
                        if name == subdir_str {
                            new_cur = Some(n.clone());
                            println!("Found existing dir {}", subdir_str);
                        }
                    }
                }
            }

            // otherwise add new
            if new_cur.is_none() {
                if let Node::Dir { ref mut childs, .. } = *cur.borrow_mut() {
                    let new_dir = Rc::new(RefCell::new(Node::Dir {
                        name: subdir_str.into(),
                        childs: vec![],
                    }));
                    childs.push(new_dir.clone());
                    new_cur = Some(new_dir);
                    println!("Add new dir {}", subdir_str);
                }
            }

            // go to new directory
            stack.push(new_cur.unwrap());
        }
        if line == "$ ls" {
            idx += 1;
            while idx < lines.len() {
                // Iterate content of current directory
                if lines[idx].starts_with('$') {
                    break;
                }
                if let Node::Dir { ref mut childs, .. } = *stack.last().unwrap().borrow_mut() {
                    if !lines[idx].starts_with("dir") {
                        let mut comp = lines[idx].split_whitespace();
                        let size: usize = comp.next().unwrap().parse().unwrap();
                        let name: String = comp.next().unwrap().into();
                        let new_file = Rc::new(RefCell::new(Node::File { name, size }));
                        childs.push(new_file);
                    }
                }
                idx += 1
            }
        }
    }

    // get root
    stack.first().unwrap().clone()
}

fn calc_sizes(node: Rc<RefCell<Node>>, sizes: &mut Vec<usize>) -> usize {
    match *node.borrow() {
        Node::File { ref size, .. } => *size,
        Node::Dir {
            ref name,
            ref childs,
        } => {
            let dir_size: usize = childs.iter().map(|c| calc_sizes(c.clone(), sizes)).sum();
            println!("Dir {:?} has size {}", name, dir_size);
            sizes.push(dir_size);
            dir_size
        }
    }
}

pub fn process_a(lines: Vec<String>) -> String {
    let tree = build_tree(lines);
    println!("{}", tree.borrow());
    let mut sizes: Vec<usize> = vec![];
    calc_sizes(tree, &mut sizes);
    println!("Dir sizes {:?}", &sizes);
    let lim_space = 100000;
    sizes
        .iter()
        .filter(|v| **v < lim_space)
        .sum::<usize>()
        .to_string()
}

pub fn process_b(lines: Vec<String>) -> String {
    let tree = build_tree(lines);
    println!("{}", tree.borrow());
    let mut sizes: Vec<usize> = vec![];
    calc_sizes(tree, &mut sizes);
    sizes.sort();
    let need_space = 30000000;
    let total_space = 70000000;
    let used_space = sizes.last().unwrap();
    println!("Dir sizes {:?}", &sizes);
    sizes
        .iter()
        .find(|v| total_space + **v >= need_space + used_space)
        .unwrap()
        .to_string()
}
