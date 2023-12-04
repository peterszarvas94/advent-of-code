use std::fs;

fn check_for_digit(substring: &str, digits: &[&str; 9]) -> (bool, usize) {
  for (i, digit) in digits.iter().enumerate() {
    if substring.contains(digit) {
      let number = i + 1;
      return (true, number);
    }
  }

  (false, 0)
}

fn main() {
  let contents = match fs::read_to_string("src/input.txt") {
    Ok(contents) => contents,
    Err(_) => panic!("Error reading file"),
  };

  let rows = contents.lines();
  let mut total: usize = 0;
  let digits: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
  ];

  for row in rows {
    // from front to back
    for (i, chr) in row.chars().enumerate() {
      if chr.is_digit(10) {
        let numeral = match chr.to_digit(10) {
          Some(digit) => digit as usize,
          None => 0,
        };
        total += numeral * 10;
        break;
      }

      let substr = &row[0..=i];
      let (found, number) = check_for_digit(substr, &digits);
      if found {
        total += number * 10;
        break;
      }
    }

    // from back to front
    for (i, chr) in row.chars().rev().enumerate() {
      if chr.is_digit(10) {
        let numeral = match chr.to_digit(10) {
          Some(digit) => digit as usize,
          None => 0,
        };
        total += numeral;
        break;
      }

      let substr = &row[row.len() - i - 1..row.len()];
      let (found, number) = check_for_digit(substr, &digits);
      if found {
        total += number;
        break;
      }
    }
  }

  println!("Total: {}", total);
}
