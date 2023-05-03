use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn win_score(enemy: &str) -> i32 {
    if enemy == "Y" {
        return 3;
    }
    if enemy == "Z" {
        return 6;
    }
    return 0;
}

fn shape_score(shape: String) -> i32 {
    if shape == "A" {
        return 1;
    }
    if shape == "B" {
        return 2;
    }
    return 3;
}

fn calc_my_shape(enemy: &str, outcome: &str) -> String {
    if enemy == "A" {
        if outcome == "X" {
            return String::from("C");
        }
        if outcome == "Y" {
            return String::from("A");
        }
        return String::from("B");
    }

    if enemy == "B" {
        if outcome == "X" {
            return String::from("A");
        }
        if outcome == "Y" {
            return String::from("B");
        }
        return String::from("C");
    }

    if outcome == "X" {
        return String::from("B");
    }
    if outcome == "Y" {
        return String::from("C");
    }
    return String::from("A");
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
    // A beats C, C beats B, B beats A
    // X lose, Y draw, Z win
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
                let win_score = win_score(arr[1]);
                let my_shape = calc_my_shape(arr[0], arr[1]);
                let shape_score = shape_score(my_shape);
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
