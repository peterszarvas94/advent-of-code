use std::{collections::HashSet, fs::read_to_string};

fn is_signal(v: &Vec<char>) -> bool {
    let mut set = HashSet::new();
    for elem in v {
        if !set.insert(elem) {
            return false;
        }
    }
    true
}

fn main() {
    // open file
    let file = read_to_string("src/input");
    let file = match file {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut v = Vec::new();

    let chars = file.chars().enumerate();

    for (i, c) in chars {
        v.push(c);
        if i >= 14 {
            v.remove(0);
            if is_signal(&v) {
                println!("{}", i + 1);
                break;
            }
        }
    }
}
