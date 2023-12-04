use std::fs;

fn main() {
  let contents = match fs::read_to_string("src/input.txt") {
    Ok(contents) => contents,
    Err(_) => panic!("Error reading file"),
  };

  let rows = contents.lines();
  let mut total = 0;

  for row in rows {
    let mut points = 0;

    let game = row.split(": ").collect::<Vec<&str>>()[1]
      .split(" | ")
      .collect::<Vec<&str>>();

    let winning = game[0]
      .split_whitespace()
      .filter_map(|n| n.parse::<i32>().ok())
      .collect::<Vec<i32>>();

    let my = game[1]
      .split_whitespace()
      .filter_map(|n| n.parse::<i32>().ok())
      .collect::<Vec<i32>>();

    for n in my.iter() {
      if winning.contains(n) {
        if points == 0 {
          points = 1;
        } else {
          points *= 2;
        }
      }
    }

    total += points;
  }

  println!("Total: {}", total);
}
