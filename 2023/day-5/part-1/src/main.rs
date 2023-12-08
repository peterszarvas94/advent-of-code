use std::fs;

#[derive(Debug)]
struct Line {
  target_start: usize,
  source_start: usize,
  range: usize,
}

#[derive(Debug)]
struct Map {
  lines: Vec<Line>,
}

fn check_map(input: usize, map: &Map) -> usize {
  println!("New map, input: {}", input);
  for line in &map.lines {
    println!(" Line: {:?}", line);
    if input < line.source_start || line.source_start + line.range - 1 < input {
      continue;
    }

    let diff = input - line.source_start;
    if diff <= line.range {
      println!("  Diff: {}", diff);
      println!("  Output: {}", line.target_start + diff);
      return line.target_start + diff;
    }
  }

  return input;
}

fn check_seed(input: usize, maps: &Vec<Map>) -> usize {
  let mut output = input;

  for map in maps {
    output = check_map(output, map);
  }

  return output;
}

fn main() {
  let contents = match fs::read_to_string("src/sample.txt") {
    Ok(contents) => contents,
    Err(_) => panic!("Error reading file"),
  };

  let rows = contents.lines();
  let mut min = 0;

  let mut maps: Vec<Map> = Vec::new();
  let mut lines_temp: Vec<Line> = Vec::new();
  let mut seeds: Vec<usize> = Vec::new();

  let mut empty_row = 0;

  // process all the maps
  for (i, row) in rows.enumerate() {
    if i == 0 {
      seeds = row
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    }

    if row == "" {
      empty_row = i;

      if lines_temp.len() > 0 {
        let map = Map { lines: lines_temp };
        maps.push(map);
        lines_temp = Vec::new();
      }
    }

    if i >= empty_row + 2 {
      let data = row
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

      let line = Line {
        target_start: data[0],
        source_start: data[1],
        range: data[2],
      };

      lines_temp.push(line);
    }
  }

  // add the last map
  if lines_temp.len() > 0 {
    let map = Map { lines: lines_temp };
    maps.push(map);
  }

  for seed in seeds {
    let output = check_seed(seed, &maps);
    if output < min || min == 0 {
      min = output;
    }
  }

  println!("Total: {}", min);
}


//tests

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_check_map() {
    let map = Map {
      lines: vec![
        Line {
          target_start: 10,
          source_start: 20,
          range: 5,
        },
        Line {
          target_start: 50,
          source_start: 30,
          range: 5,
        },
      ],
    };

    assert_eq!(check_map(30, &map), 50);
  }

  #[test]
  fn test_check_seed() {
    let map1 = Map {
      lines: vec![
        Line {
          target_start: 10,
          source_start: 20,
          range: 5,
        },
        Line {
          target_start: 50,
          source_start: 30,
          range: 5,
        },
      ],
    };

    let map2 = Map {
      lines: vec![
        Line {
          target_start: 100,
          source_start: 200,
          range: 5,
        },
        Line {
          target_start: 500,
          source_start: 300,
          range: 5,
        },
      ],
    };

    let maps = vec![map1, map2];

    assert_eq!(check_seed(305, &maps), 505);
  }
}
