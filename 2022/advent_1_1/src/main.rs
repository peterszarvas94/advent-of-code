use std::fs::File;
use std::io::{self, BufRead, BufReader};

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

    let mut sum: u64 = 0;
    let mut max: u64 = 0;

    // read lines
    for line in reader.lines() {
        match line {
            Ok(l) => {
                if l.is_empty() {
                    if sum > max {
                        max = sum;
                    }
                    sum = 0;
                } else {
                    let num: u64 = l.parse().unwrap();
                    sum += num;
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                return Err(e);
            }
        }
    }

    println!("max: {}", max);

    // default return
    Ok(())
}
