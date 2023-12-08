use std::fs;

fn main() {
  let contents = match fs::read_to_string("src/input.txt") {
    Ok(contents) => contents,
    Err(_) => panic!("Error reading file"),
  };

  let mut rows = contents.lines();

  let times = rows
    .clone()
    .nth(0)
    .unwrap()
    .split(":")
    .nth(1)
    .unwrap()
    .split_whitespace()
    .filter_map(|x| x.parse::<usize>().ok())
    .collect::<Vec<usize>>();

  let records = rows
    .nth(1)
    .unwrap()
    .split(":")
    .nth(1)
    .unwrap()
    .split_whitespace()
    .filter_map(|x| x.parse::<usize>().ok())
    .collect::<Vec<usize>>();

  let mut winning_games: Vec<usize> = Vec::new();

  for (i, time) in times.iter().enumerate() {
    let record = records[i];
    let mut wins: Vec<usize> = Vec::new();
    for j in 0..=*time {
      let distance = j * (*time - j);
      if distance > record {
        wins.push(distance);
      }
    }
    winning_games.push(wins.len());
  }

  let result = winning_games.iter().fold(1, |acc, x| acc * x);

  // println!("Times: {:?}", times);
  // println!("Distances: {:?}", records);

  println!("Resut: {}", result);
}
