use std::{collections::HashSet, fs};

fn main() {
  let contents = match fs::read_to_string("src/input.txt") {
    Ok(contents) => contents,
    Err(_) => panic!("Error reading file"),
  };

  // 1) expand in empty col/row
  // 2) collect ID and (x, y) coords
  // 3) check 1. for 2., 3., 4. (there is 4 points)
  //    check 2. for 3., 4.
  //    check 3. for 4.
  //    don't check 4.
  // 4) distance is max(ay, by)-min(ay, by) + max(ax, bx)-min(ax, bx)
  // 5) add distance to total

  let rows = contents.lines();
  let mut map: Vec<Vec<char>> = Vec::new();

  let mut star_indexes: HashSet<usize> = HashSet::new();

  for row in rows {
    map.push(row.chars().collect());
    println!("{:?}", row);
    if !row.contains("#") {
      // duplicate row
      map.push(row.chars().collect());
      println!("{:?}", row);
    }

    for (i, c) in row.chars().enumerate() {
      if c == '#' {
        star_indexes.insert(i);
      }
    }
  }

  let mut expanded_map: Vec<Vec<char>> = Vec::new();

  for row in map {
    let mut expanded_row: Vec<char> = Vec::new();
    for (i, c) in row.iter().enumerate() {
      expanded_row.push(*c);
      if !star_indexes.contains(&i) {
        // duplicate col
        expanded_row.push(*c);
      }
    }
    println!("{:?}", expanded_row.clone().into_iter().collect::<String>());
    expanded_map.push(expanded_row);
  }

  let mut sorted = star_indexes.iter().cloned().collect::<Vec<usize>>();
  sorted.sort();

  println!("Indexes {:?}", sorted);

  // collect new star coords
  let mut star_coords: Vec<(usize, usize)> = Vec::new();

  for (i, row) in expanded_map.iter().enumerate() {
    for (j, c) in row.iter().enumerate() {
      if *c == '#' {
        star_coords.push((i, j));
      }
    }
  }

  println!("Star coords {:?}", star_coords);

  let mut total = 0;

  for (i, star) in star_coords.iter().enumerate() {
    let (x1, y1) = star;
    // get other star ditance, also avoid duplicates by not checking backwards
    for j in i + 1..star_coords.len() {
      let (x2, y2) = match star_coords.get(j) {
        Some(coords) => coords,
        None => panic!("Error getting other star coords"),
      };
      let dx = x1.max(x2) - x1.min(x2);
      let dy = y1.max(y2) - y1.min(y2);
      println!("{:?} {:?} dx {:?} dy {:?}", (x1, y1), (x2, y2), dx, dy);
      let distance = dx + dy;
      total += distance;
    }
  }

  println!("Total {:?}", total);
}
