use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn win_score(first: &str, second: &str) -> i32 {
    if (first == "A" && second == "Y")
        || (first == "B" && second == "Z")
        || (first == "C" && second == "X")
    {
        return 6;
    }

    if (first == "A" && second == "X")
        || (first == "B" && second == "Y")
        || (first == "C" && second == "Z")
    {
        return 3;
    }

    return 0;
}

fn shape_score(shape: &str) -> i32 {
    if shape == "X" {
        return 1;
    }

    if shape == "Y" {
        return 2;
    }

    if shape == "Z" {
        return 3;
    }

    return 0;
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

    // 6 if win, 3 if draw, 0 if loss
    // A > Z, B > X, C > Y
    // 1 X, 2 Y, 3 Z

    let mut total_score = 0;

    // read lines
    for line in reader.lines() {
        match line {
            Ok(l) => {
                // split line into array
                let arr = l.split_whitespace().collect::<Vec<&str>>();
                println!("{:?}", arr);

                // check if win, draw, or loss
                let win_score = win_score(arr[0], arr[1]);
                let shape_score = shape_score(arr[1]);
                let match_score = win_score + shape_score;

                total_score += match_score;
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                return Err(e);
            }
        }
    }

    println!("total: {}", total_score);

    // default return
    Ok(())
}
