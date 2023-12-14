use std::fs;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Dir {
  North,
  South,
  East,
  West,
  Start,
}

// collect loop coords
// which is not loop, make a .
// in row, count intersections (ray casting)
// odd intersections are inside, even are outside

fn get_pipe_from_char(c: char) -> Option<[Dir; 2]> {
  let dirs = match c {
    '|' => [Dir::North, Dir::South],
    '-' => [Dir::West, Dir::East],
    '7' => [Dir::South, Dir::West],
    'F' => [Dir::South, Dir::East],
    'J' => [Dir::North, Dir::West],
    'L' => [Dir::North, Dir::East],
    'S' => [Dir::Start, Dir::Start],
    _ => return None,
  };
  return Some(dirs);
}

fn get_outlet(pipe: [Dir; 2], inlet: Dir) -> Option<Dir> {
  if pipe[0] == inlet || pipe[0] == Dir::Start {
    return Some(pipe[1]);
  } else if pipe[1] == inlet {
    return Some(pipe[0]);
  } else {
    return None;
  }
}

fn get_opposite_dir(dir: Dir) -> Dir {
  match dir {
    Dir::North => Dir::South,
    Dir::South => Dir::North,
    Dir::East => Dir::West,
    Dir::West => Dir::East,
    Dir::Start => Dir::Start,
  }
}

fn main() {
  let contents = match fs::read_to_string("src/input.txt") {
    Ok(contents) => contents,
    Err(_) => panic!("Error reading file"),
  };

  let rows = contents.lines();
  let mut total: usize = 0;
  let mut map: Vec<Vec<char>> = rows
    .map(|row| {
      let chars: Vec<char> = row.chars().collect();
      return chars;
    })
    .collect();

  // get 'S' char x,y
  let mut start: [usize; 2] = [0, 0];
  for (y, row) in map.iter().enumerate() {
    for (x, ch) in row.iter().enumerate() {
      if *ch == 'S' {
        start = [x, y];
      }
    }
  }

  let mut prev_outlet = Dir::Start;
  let mut current_coords = start;

  let mut loop_coords: Vec<[usize; 2]> = Vec::new();
  loop_coords.push(current_coords);

  loop {
    // look north
    if prev_outlet == Dir::South || prev_outlet == Dir::Start {
      let inlet = match prev_outlet {
        Dir::Start => Dir::South,
        _ => prev_outlet,
      };

      let x = current_coords[0];
      match current_coords[1].checked_sub(1) {
        Some(y) => match map
          .get(y)
          .and_then(|row| row.get(x))
          .and_then(|c| get_pipe_from_char(*c))
          .and_then(|pipe| get_outlet(pipe, inlet))
        {
          Some(outlet) => {
            let opposite = get_opposite_dir(outlet);
            loop_coords.push([x, y]);
            prev_outlet = opposite;
            current_coords = [x, y];
            if outlet == Dir::Start {
              break;
            }
            continue;
          }
          None => {}
        },
        None => {}
      };
    }

    // look east
    if prev_outlet == Dir::West || prev_outlet == Dir::Start {
      let inlet = match prev_outlet {
        Dir::Start => Dir::West,
        _ => prev_outlet,
      };

      let x = current_coords[0] + 1;
      let y = current_coords[1];

      match map
        .get(y)
        .and_then(|row| row.get(x))
        .and_then(|c| get_pipe_from_char(*c))
        .and_then(|pipe| get_outlet(pipe, inlet))
      {
        Some(outlet) => {
          let opposite = get_opposite_dir(outlet);
          loop_coords.push([x, y]);
          // total += 1;
          prev_outlet = opposite;
          current_coords = [x, y];
          if outlet == Dir::Start {
            break;
          }
          continue;
        }
        None => {}
      }
    }

    // look south
    if prev_outlet == Dir::North || prev_outlet == Dir::Start {
      let inlet = match prev_outlet {
        Dir::Start => Dir::North,
        _ => prev_outlet,
      };
      let x = current_coords[0];
      let y = current_coords[1] + 1;

      match map
        .get(y)
        .and_then(|row| row.get(x))
        .and_then(|c| get_pipe_from_char(*c))
        .and_then(|pipe| get_outlet(pipe, inlet))
      {
        Some(outlet) => {
          let opposite = get_opposite_dir(outlet);
          loop_coords.push([x, y]);
          prev_outlet = opposite;
          current_coords = [x, y];
          if outlet == Dir::Start {
            break;
          }
          continue;
        }
        None => {}
      }
    }

    // look west
    if prev_outlet == Dir::East || prev_outlet == Dir::Start {
      let inlet = match prev_outlet {
        Dir::Start => Dir::East,
        _ => prev_outlet,
      };

      let y = current_coords[1];
      match current_coords[0].checked_sub(1) {
        Some(x) => match map
          .get(y)
          .and_then(|row| row.get(x))
          .and_then(|c| get_pipe_from_char(*c))
          .and_then(|pipe| get_outlet(pipe, inlet))
        {
          Some(outlet) => {
            let opposite = get_opposite_dir(outlet);
            loop_coords.push([x, y]);
            prev_outlet = opposite;
            current_coords = [x, y];
            if outlet == Dir::Start {
              break;
            }
            continue;
          }
          None => {}
        },
        None => {}
      };
    }
  }

  // replace all non-loop chars with .
  for (y, row) in map.iter_mut().enumerate() {
    for (x, ch) in row.iter_mut().enumerate() {
      if !loop_coords.contains(&[x, y]) {
        *ch = '.';
      }
    }
  }

  // solve
  for row in map.iter() {
    let mut inside = false;
    let mut f_started = false;
    let mut l_started = false;
    for ch in row.iter() {
      match *ch {
        '|' => {
          inside = !inside;
        }
        '.' => {
          if inside {
            total += 1;
          }
        }
        'F' => {
          f_started = true;
        }
        'L' => {
          l_started = true;
        }
        'J' => {
          if f_started {
            inside = !inside;
            f_started = false;
          } else if l_started {
            l_started = false;
          }
        }
        '7' => {
          if l_started {
            inside = !inside;
            l_started = false;
          } else if f_started {
            f_started = false;
          }
        }
        _ => {}
      }
    }
  }

  print_map(&map);
  println!("Total: {}", total);
}

fn print_map(map: &Vec<Vec<char>>) {
  for row in map.iter() {
    for ch in row.iter() {
      match *ch {
        '7' => print!("⌝"),
        'F' => print!("⌜"),
        'J' => print!("⌟"),
        'L' => print!("⌞"),
        'S' => print!("S"),
        '|' => print!("|"),
        '-' => print!("—"),
        _ => print!("."),
      }
    }
    println!();
  }
}
