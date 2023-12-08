use std::fs;

fn main() {
  let contents = match fs::read_to_string("src/input.txt") {
    Ok(contents) => contents,
    Err(_) => panic!("Error reading file"),
  };

  let mut rows = contents.lines();

  let time = rows
    .clone()
    .nth(0)
    .unwrap()
    .split(":")
    .nth(1)
    .unwrap()
    .chars()
    .filter(|c| c.is_numeric())
    .collect::<String>()
    .parse::<usize>()
    .unwrap();

  let record = rows
    .nth(1)
    .unwrap()
    .split(":")
    .nth(1)
    .unwrap()
    .chars()
    .filter(|c| c.is_numeric())
    .collect::<String>()
    .parse::<usize>()
    .unwrap();

  let mut wins: Vec<usize> = Vec::new();
  for j in 0..=time {
    let speed = &time - j;
    let distance = j * speed;
    if distance > record {
      wins.push(distance);
    }
  }

  let result = wins.len();

  println!("Resut: {}", result);
}
