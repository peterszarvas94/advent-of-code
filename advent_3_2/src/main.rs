use std::fs::File;
use std::io::{BufRead, BufReader};

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

fn find_common_item(a: &String, b: &String, c: &String) -> Option<char> {
    let a_and_b = a.chars().filter(|&ch| b.contains(ch)).collect::<String>();
    let all = a_and_b
        .chars()
        .filter(|&ch| c.contains(ch))
        .collect::<String>();
    return all.chars().nth(0);
}

fn main() {
    // open file
    let file = match File::open("src/input") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mut group: Vec<String> = Vec::new();
    let mut count = 0;

    for line in reader.lines() {
        // read line
        let current_line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue;
            }
        };

        // push line one and two
        if count < 2 {
            group.push(current_line);
            count += 1;
            continue;
        }

        // push line three
        group.push(current_line);

        // find common items
        let common = find_common_item(&group[0], &group[1], &group[2]);
        let group_char = match common {
            Some(c) => c,
            None => ' ',
        };

        // convert char to num
        let num = char_to_num(group_char);
        let group_num = match num {
            Some(n) => n,
            None => 0,
        };

        // add to sum
        sum += group_num;
        group.clear();
        count = 0;
    }

    println!("sum: {}", sum);
}
