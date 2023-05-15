use std::fs::read_to_string;

fn create_row_map_from_row(row: &Vec<u32>, default: &Vec<u32>) -> Vec<u32> {
  let mut map = default.clone();

  row.iter().enumerate().for_each(|(i, tree)| {
    let mut left_points = 0;
    let trees_to_left = &row[..i];

    for other_tree in trees_to_left.iter().rev() {
      left_points += 1;
      if other_tree < tree {
        continue;
      }
      if other_tree >= tree {
        break;
      }
    }
    map[i] *= left_points;

    let mut right_points = 0;
    let trees_to_right = &row[i + 1..];

    for other_tree in trees_to_right.iter() {
      right_points += 1;

      if other_tree < tree {
        continue;
      }
      if other_tree >= tree {
        break;
      }
    }

    map[i] *= right_points;
  });

  map
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
  //open file
  let file = match read_to_string("src/input") {
    Ok(f) => f,
    Err(e) => {
      eprintln!("Error reading file: {}", e);
      return;
    }
  };

  // parse file
  let matrix: Vec<Vec<u32>> = file
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|c| match c.to_digit(10) {
          Some(d) => d,
          None => panic!("Invalid input"),
        })
        .collect()
    })
    .collect();

  // create map
  let default_row = vec![1; matrix[0].len()];
  let default_map = vec![default_row; matrix.len()];

  // modify map rows
  let mut map = default_map.clone();
  default_map.iter().enumerate().for_each(|(i, row)| {
    let row_map = create_row_map_from_row(&matrix[i], &row);
    map[i] = row_map;
  });

  // rotate matrix and map
  let rotated = rotate_matrix(&matrix);
  let map_rotated = rotate_matrix(&map);

  // modify map cols
  let mut final_map = map_rotated.clone();
  map_rotated.iter().enumerate().for_each(|(i, row)| {
    let row_map = create_row_map_from_row(&rotated[i], &row);
    final_map[i] = row_map;
  });

  // max value
  let max = match final_map.iter().flatten().max() {
    Some(m) => m,
    None => panic!("No max value found"),
  };

  println!("Max value: {}", max);
}
