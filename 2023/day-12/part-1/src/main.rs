use std::fs;

#[derive(Debug, Clone, Copy)]
enum Spring {
  Working,
  Broken,
}

impl Spring {
  fn new_from_char(c: &char) -> Spring {
    match c {
      '.' => Spring::Working,
      '#' => Spring::Broken,
      _ => panic!("Error creating Spring"),
    }
  }
}

fn map_from_string(s: &str) -> Vec<Spring> {
  let mut map: Vec<Spring> = Vec::new();

  for c in s.chars() {
    match c {
      '.' | '#' => {
        let spring = Spring::new_from_char(&c);
        map.push(spring);
      }
      _ => panic!("Error"),
    }
  }

  return map;
}

fn get_maps(s: &str, current: &mut String, maps: &mut Vec<Vec<Spring>>) {
  if current.len() == s.len() {
    let map = map_from_string(&current);
    return maps.push(map);
  }

  let c = match s.chars().nth(current.len()) {
    Some(c) => c,
    None => panic!("Error getting char"),
  };

  match c {
    '?' => {
      current.push('.');
      get_maps(s, current, maps);
      current.pop();

      current.push('#');
      get_maps(s, current, maps);
      current.pop();
    }
    '.' | '#' => {
      current.push(c);
      get_maps(s, current, maps);
      current.pop();
    }
    _ => panic!("Error matching char"),
  }
}

fn count_broken_groups(springs: &[Spring]) -> Vec<usize> {
  let mut count = Vec::new();
  let mut current_count = 0;

  for &spring in springs {
    match spring {
      Spring::Working => {
        if current_count > 0 {
          count.push(current_count);
          current_count = 0;
        }
      }
      Spring::Broken => current_count += 1,
    }
  }

  if current_count > 0 {
    count.push(current_count);
  }

  return count;
}

fn main() {
  let contents = match fs::read_to_string("src/input.txt") {
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

    // println!("Map: {} Numbers: {:?}", char_map, numbers);

    let mut maps: Vec<Vec<Spring>> = Vec::new();
    let mut current = String::new();

    get_maps(char_map, &mut current, &mut maps);

    let mut row_total = 0;
    for map in maps {
      let count = count_broken_groups(&map);
      if count == numbers {
        row_total += 1;
        // println!("Match: {:?} count: {:?}", map, count);
      }
    }
    println!("Row total: {}", row_total);
    total += row_total;
  }

  println!("Total: {}", total);
}
