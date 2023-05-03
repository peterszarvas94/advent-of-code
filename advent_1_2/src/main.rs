use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, ErrorKind};
use std::num::ParseIntError;

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

    let mut current: u64 = 0;
    let mut arr: [u64; 3] = [0, 0, 0];

    // read lines
    for line in reader.lines() {
        match line {
            Ok(l) => {
                if l.is_empty() {
                    let mut min_idx = 0;
                    for (idx, &val) in arr.iter().enumerate() {
                        if val < arr[min_idx] {
                            min_idx = idx;
                        }
                    }
                    if current > arr[min_idx] {
                        arr[min_idx] = current;
                    }
                    current = 0;
                } else {
                    let num: Result<u64, ParseIntError> = l.parse();
                    match num {
                        Ok(n) => {
                            current += n;
                        }
                        Err(e) => {
                            eprintln!("Error parsing line: {}", e);
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                "Failed to parse integer",
                            ));
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                return Err(e);
            }
        }
    }
    let sum: u64 = arr.iter().sum();
    println!("max: {}", sum);

    // default return
    Ok(())
}
