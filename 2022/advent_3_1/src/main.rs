use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn char_to_num(c: char) -> Option<u32> {
    let is_lowercase = c.is_ascii_lowercase();
    let is_uppercase = c.is_ascii_uppercase();
    let digit = c.to_digit(36);

    match digit {
        Some(d) => {
            if is_lowercase {
                return Some(d - 9);
            }
            if is_uppercase {
                return Some(d - 9 + 26);
            }
            Some(d)
        }
        None => None,
    }
}

fn split_string(s: String) -> (Vec<char>, Vec<char>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    let chars = s.chars().collect::<Vec<char>>();

    for i in 0..chars.len() {
        if i < chars.len() / 2 {
            let c = chars[i];
            left.push(c);
            continue;
        }

        let c = chars[i];
        right.push(c);
    }
    (left, right)
}

fn find_common_items(left: Vec<char>, right: Vec<char>) -> Vec<char> {
    let mut common = Vec::new();

    for i in 0..left.len() {
        let c = left[i];
        if right.contains(&c) {
            common.push(c);
        }
    }

    common
}

fn main() -> io::Result<()> {
    // open file
    let file = match File::open("src/input") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return Err(e);
        }
    };
    let reader = BufReader::new(file);

    let mut sum = 0;

    // read lines
    for line in reader.lines() {
        match line {
            Ok(l) => {
                let (left, right) = split_string(l);
                let common_items = find_common_items(left, right);
                let first = common_items[0];
                let num = char_to_num(first);
                match num {
                    Some(n) => {
                        sum += n;
                    }
                    None => {}
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                return Err(e);
            }
        }
    }

    println!("sum: {}", sum);

    // default return
    Ok(())
}
