use std::fs::read_to_string;

fn create_row_map_from_row(input: &Vec<u32>, target: &Vec<u32>) -> Vec<u32> {
  let mut biggest = 0;
  let mut map = target.clone();
  for (i, x) in input.iter().enumerate() {
    if *x > biggest {
      biggest = *x;
      map[i] = 1;
      continue;
    }
  }
  // reverse:
  biggest = 0;
  for (i, x) in input.iter().enumerate().rev() {
    if *x > biggest {
      biggest = *x;
      map[i] = 1;
      continue;
    }
  }

  return map;
}

fn rotate_matrix(matrix: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
  let rows = matrix.len();
  let cols = matrix[0].len();

  let mut rotated = vec![vec![0; rows]; cols];

  for i in 0..rows {
    for j in 0..cols {
      rotated[j][rows - i - 1] = matrix[i][j];
    }
  }

  return rotated;
}

fn main() {
  // open file
  let file = match read_to_string("src/input") {
    Ok(f) => f,
    Err(e) => {
      eprintln!("Error reading file: {}", e);
      return;
    }
  };

  let matrix: Vec<Vec<u32>> = file
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|c| match c.to_digit(10) {
          Some(d) => d + 1,
          None => panic!("Invalid input"),
        })
        .collect()
    })
    .collect();

  let empty_row = vec![0; matrix[0].len()];
  let empty_map = vec![empty_row; matrix.len()];

  let mut map = empty_map.clone();
  empty_map.iter().enumerate().for_each(|(i, row)| {
    let row_map = create_row_map_from_row(&matrix[i], &row);
    map[i] = row_map;
  });

  let rotated = rotate_matrix(&matrix);
  let map_rotated = rotate_matrix(&map);

  let mut final_map = map_rotated.clone();
  map_rotated.iter().enumerate().for_each(|(i, row)| {
    let row_map = create_row_map_from_row(&rotated[i], &row);
    final_map[i] = row_map;
  });

  let sum: u32 = final_map
    .iter()
    .fold(0, |acc, x| acc + x.iter().fold(0, |acc, y| acc + y));

  println!("Sum: {}", sum);
}
