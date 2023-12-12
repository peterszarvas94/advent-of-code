use std::fs;

fn main() {
  let contents = match fs::read_to_string("src/input.txt") {
    Ok(contents) => contents,
    Err(_) => panic!("Error reading file"),
  };

  let rows = contents.lines();
  let mut total = 0;

  for row in rows {
    let numbers = row
      .split_whitespace()
      .map(|x| match x.parse::<i64>() {
        Ok(x) => x,
        Err(_) => panic!("Error parsing number"),
      })
      .collect::<Vec<i64>>();

    let mut zeros = check_zeros(&numbers);
    let mut map = vec![numbers];

    while !zeros {
      let last = match map.last() {
        Some(last) => last,
        None => panic!("Error getting last element i map"),
      };

      let new_sequence = sequence(last);
      map.push(new_sequence.clone());
      zeros = check_zeros(&new_sequence);
    }

    let mut prev = 0;
    for sequence in map.iter_mut().rev() {
      let s_cloned = sequence.clone();
      let current = match s_cloned.first() {
        Some(first) => first,
        None => panic!("Error getting last element in sequence"),
      };

      let next = current - prev;
      sequence.insert(0, next);

      prev = next;
    }

    let first_seq = match map.first() {
      Some(first) => first,
      None => panic!("Error getting last sequence in map"),
    };

    let first_num = match first_seq.first() {
      Some(last) => last,
      None => panic!("Error getting last element in sequence"),
    };

    total += first_num;
  }

  println!("Total: {}", total);
}

// utils

fn sequence(numbers: &Vec<i64>) -> Vec<i64> {
  let new_numbers = numbers
    .clone()
    .windows(2)
    .map(|pair| pair[1] - pair[0])
    .collect();

  return new_numbers;
}

fn check_zeros(numbers: &Vec<i64>) -> bool {
  // if any of the numbers is not 0, return false
  if numbers.iter().any(|&x| x != 0) {
    return false;
  }
  return true;
}
