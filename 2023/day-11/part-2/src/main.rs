use std::{collections::HashSet, fs};

#[derive(Debug, Clone)]
struct Row {
  y: usize,
  chs: Vec<char>,
  count: usize,
}

#[derive(Debug, Clone)]
struct Col {
  x: usize,
  count: usize,
}

impl Col {
  fn set_count(&mut self, count: usize) {
    self.count = count;
  }
}

#[derive(Debug, Clone)]
struct Map {
  rows: Vec<Row>,
  cols: Vec<Col>,
}

impl Map {
  fn new() -> Map {
    Map {
      rows: Vec::new(),
      cols: Vec::new(),
    }
  }

  fn add_row(&mut self, row: Row) {
    self.rows.push(row);
  }

  fn add_col(&mut self, col: Col) {
    self.cols.push(col);
  }

  fn print(&self) {
    // print row data
    println!("Rows:");
    for row in &self.rows {
      let chs: Vec<char> = row.chs.iter().cloned().collect();
      let s: String = chs.into_iter().collect();
      println!("{} ({}, {})", s, row.y, row.count);
    }

    // print col data
    println!("\nCols:");
    for col in &self.cols {
      print!("({}, {}) ", col.x, col.count);
    }
    println!("\n");
  }

  fn get_stars(&self) -> Vec<(usize, usize)> {
    let mut stars: Vec<(usize, usize)> = Vec::new();
    for row in &self.rows {
      for (x, c) in row.chs.iter().enumerate() {
        if *c == '#' {
          stars.push((row.y, x));
        }
      }
    }
    return stars;
  }

  fn get_dx(&self, start: &usize, end: &usize) -> usize {
    let mut dx = 0;
    for row in &self.rows[*start..*end] {
      dx += row.count;
    }
    return dx;
  }

  fn get_dy(&self, start: &usize, end: &usize) -> usize {
    let mut dy = 0;
    for col in &self.cols[*start..*end] {
      dy += col.count;
    }
    return dy;
  }
}

fn main() {
  let contents = match fs::read_to_string("src/input.txt") {
    Ok(contents) => contents,
    Err(_) => panic!("Error reading file"),
  };

  let rows = contents.lines();
  let mut map: Map = Map::new();

  let mut star_x_position: HashSet<usize> = HashSet::new();

  for (y, row) in rows.enumerate() {
    let chs: Vec<char> = row.chars().collect();
    let mut count = 1_000_000;
    for (x, c) in row.chars().enumerate() {
      if y == 0 {
        // fill col data
        let col: Col = Col { x, count: 1 };
        map.add_col(col);
      }

      // "copy" row 1_000_000 times
      if c == '#' {
        star_x_position.insert(x);
        count = 1;
      }
    }

    let map_row: Row = Row { y, chs, count };
    map.add_row(map_row);
  }

  // { i: 0, count: 1 }
  // let mut expanded_map: Vec<Vec<char>> = Vec::new();

  for (x, col) in map.cols.iter_mut().enumerate() {
    if !star_x_position.contains(&x) {
      // "copy" col 1_000_000 times
      col.set_count(1_000_000);
    }
  }

  map.print();

  let stars = map.get_stars();
  println!("Stars: {:?}", stars);

  let mut total = 0;

  for (i, star) in stars.iter().enumerate() {
    let (x1, y1) = star;
    // get other star ditance, also avoid duplicates by not checking backwards
    for j in i + 1..stars.len() {
      let (x2, y2) = match stars.get(j) {
        Some(coords) => coords,
        None => panic!("Error getting other star coords"),
      };
      println!("({}, {}) ({}, {})", x1, y1, x2, y2);
      let dx = map.get_dx(x1.min(x2), x1.max(x2));
      println!("dx: {}", dx);
      let dy = map.get_dy(y1.min(y2), y1.max(y2));
      println!("dy: {}", dy);
      let distance = dx + dy;
      println!("Distance: {}", distance);
      total += distance;
    }
  }

  println!("Total: {}", total);
}
