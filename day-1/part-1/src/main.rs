use std::fs;

fn main() {
  let contents = match fs::read_to_string("src/input.txt") {
    Ok(contents) => contents,
    Err(_) => panic!("Error reading file"),
  };

  let rows = contents.lines();
  let mut total = 0;

  for row in rows {
    let first_digit = match row.chars().find(|c| c.is_digit(10)).and_then(|c| c.to_digit(10)) {
      Some(digit) => digit,
      None => 0,
    };
    let last_digit = match row.chars().rev().find(|c| c.is_digit(10)).and_then(|c| c.to_digit(10)) {
      Some(digit) => digit,
      None => 0,
    };
    let my_number = first_digit * 10 + last_digit;
    total += my_number;
  }

  println!("Total: {}", total);
}
