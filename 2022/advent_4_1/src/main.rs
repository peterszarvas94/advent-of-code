use std::fs::read_to_string;

// check if a range is contained within another
fn check_contain(a: (i32, i32), b: (i32, i32)) -> bool {
    if (a.0 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.1 <= a.1) {
        return true;
    }
    false
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

    let mut how_many = 0;

    // read file line by line
    file.lines().for_each(|line| {
        let sections = line.split(",").collect::<Vec<&str>>();
        let mut ranges: Vec<(i32, i32)> = Vec::new();

        sections.iter().for_each(|section| {
            let part = section.split("-").collect::<Vec<&str>>();
            let start = match part[0].parse::<i32>() {
                Ok(r) => r,
                Err(_) => 0,
            };
            let end = match part[1].parse::<i32>() {
                Ok(r) => r,
                Err(_) => 0,
            };

            let range: (i32, i32) = (start, end);
            ranges.push(range);
        });

        // check if one range is contained in the other
        if check_contain(ranges[0], ranges[1]) {
            how_many += 1;
        }
    });

    println!("how_many: {}", how_many);
}
