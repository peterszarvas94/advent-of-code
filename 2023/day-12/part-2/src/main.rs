use std::fs;

// part two is unfinished, I thought it is working
// its slow and only works for the first 5 out of 6 samples
// should use something like combinatorics possibly

// get_maps but not recursive:
fn get_maps(s: &str, numbers: &Vec<usize>) -> Vec<String> {
  let mut maps: Vec<String> = Vec::new();
  let mut stack: Vec<String> = Vec::new();
  stack.push(String::new());

  while !stack.is_empty() {
    let mut map = match stack.pop() {
      Some(map) => map,
      None => panic!("Error getting map"),
    };
    let correct = check_map(&map, numbers, true);

    if !correct {
      continue;
    }

    let len = map.len();

    if len == s.len() {
      if check_map(&map, numbers, false) {
        maps.push(map);
      }
      continue;
    }

    let c = match s.chars().nth(len) {
      Some(c) => c,
      None => panic!("Error getting char"),
    };

    match c {
      '?' => {
        map.push('.');
        stack.push(map.clone());
        map.pop();

        map.push('#');
        stack.push(map.clone());
        map.pop();
      }
      '.' | '#' => {
        map.push(c);
        stack.push(map.clone());
        map.pop();
      }
      _ => panic!("Error matching char"),
    }
  }

  return maps;
}

// #.##. == (1, 2)
fn check_map(map: &str, numbers: &Vec<usize>, partial: bool) -> bool {
  let mut counts: Vec<usize> = Vec::new();
  let mut count: usize = 0;

  for c in map.chars() {
    match c {
      '#' => {
        count += 1;
      }
      '.' => {
        if count > 0 {
          counts.push(count);
          count = 0;
        }
      }
      _ => panic!("Error matching char"),
    }
  }

  if count > 0 {
    counts.push(count);
  }

  if !partial && counts.len() != numbers.len() {
    println!("{} {:?} {}", map, numbers, false);
    return false;
  }

  for (i, count) in counts.iter().enumerate() {
    let number = match numbers.get(i) {
      Some(n) => n,
      None => {
        println!("{} {:?} {}", map, numbers, false);
        return false;
      }
    };
    if partial && count > number {
      println!("{} {:?} {}", map, numbers, false);
      return false;
    }
    if !partial && count != number {
      println!("{} {:?} {}", map, numbers, false);
      return false;
    }
  }

  let a = match partial {
    true => "maybe",
    false => "true",
  };
  println!("{} {:?} {}", map, numbers, a);
  return true;
}

fn main() {
  let contents = match fs::read_to_string("src/sample.txt") {
    Ok(contents) => contents,
    Err(_) => panic!("Error reading file"),
  };

  let rows = contents.lines();
  let mut total: usize = 0;

  for row in rows {
    let data = row.split_whitespace().collect::<Vec<&str>>();
    let char_map = match data.get(0) {
      Some(m) => m,
      None => panic!("Error getting map"),
    };

    // copy char_map 5 times
    // #. -> #.?#.?#.?#.?#
    let mut char_map_extended = String::new();
    for _ in 0..5 {
      char_map_extended.push_str(char_map);
      char_map_extended.push_str("?");
    }

    let numbers_str = match data.get(1) {
      Some(n) => n,
      None => panic!("Error getting numbers"),
    };

    let numbers = numbers_str
      .split(",")
      .map(|n| match n.parse::<usize>() {
        Ok(n) => n,
        Err(_) => panic!("Error parsing number"),
      })
      .collect::<Vec<usize>>();

    // copy numbers 5 times
    // 1,2 -> 1,2,1,2,1,2,1,2,1,2
    let mut numbers_extended: Vec<usize> = Vec::new();
    for _ in 0..5 {
      numbers_extended.extend_from_slice(&numbers);
    }

    // let mut maps: Vec<String> = Vec::new();
    // let mut current = String::new();

    let maps = get_maps(&char_map, &numbers);
    let row_total = maps.len();
    println!("{}", row_total);
    total += row_total;
  }

  println!("Total: {}", total);
}
