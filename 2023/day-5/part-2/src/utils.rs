#[derive(Debug)]
pub struct Line {
  pub target_start: i64,
  pub source_start: i64,
  pub range: i64,
}

#[derive(Debug)]
pub struct Map {
  pub lines: Vec<Line>,
}

pub fn overlap(range: (i64, i64), filter: (i64, i64)) -> Option<(i64, i64)> {
  let (r_start, r_end) = range;
  let (f_start, f_end) = filter;

  if r_start == r_end && r_start <= f_start && f_start <= r_end {
    return Some((r_start, r_end));
  }

  if f_start < r_end && r_start < f_end {
    let start = r_start.max(f_start);
    let end = r_end.min(f_end);
    return Some((start, end));
  }

  return None;
}

pub fn subtracts((a_start, a_end): (i64, i64), (b_start, b_end): (i64, i64)) -> Vec<(i64, i64)> {
  let mut output: Vec<(i64, i64)> = Vec::new();

  if a_start < b_start {
    let end = a_end.min(b_start - 1);
    output.push((a_start, end));
  }

  if b_end < a_end {
    let start = a_start.max(b_end + 1);
    output.push((start, a_end));
  }

  return output;
}

pub fn range_line(range: (i64, i64), line: &Line) -> Option<Vec<(i64, i64)>> {
  let mut output: Vec<(i64, i64)> = Vec::new();
  let overlap = overlap(
    range,
    (line.source_start, line.source_start + line.range - 1),
  );

  match overlap {
    Some(overlap) => {
      let diff = line.target_start - line.source_start;

      let converted = (overlap.0 + diff, overlap.1 + diff);
      println!("- - - overlap: {:?}, diff: {}, converted: {:?}", overlap, diff, converted);
      output.push(converted);

      let subtracts = subtracts(
        range,
        (line.source_start, line.source_start + line.range - 1),
      );
      for s in subtracts {
        println!("- - - subtract: {:?}", s);
        output.push(s);
      }
      return Some(output);
    }
    None => {
      println!("- - - no overlap");
      return None;
    }
  }
}

pub fn range_map(range: (i64, i64), map: &Map) -> Vec<(i64, i64)> {
  for line in &map.lines {
    println!("- - checking line: {:?}", (line.source_start, line.source_start + line.range - 1));
    let converted = range_line(range, line);
    match converted {
      Some(mut converted) => {
        converted.sort_by(|a, b| a.0.cmp(&b.0));
        return converted;
      }
      None => {}
    }
  }

  println!("- - no lines matched, returning original range");
  let output = vec![range];
  return output;
}

pub fn ranges_map(ranges: Vec<(i64, i64)>, map: &Map) -> Vec<(i64, i64)> {
  let mut output: Vec<(i64, i64)> = Vec::new();

  for range in ranges {
    println!("- checking range: {:?}", range);
    let converted = range_map(range, map);
    for c in converted {
      output.push(c);
    }
  }

  output.sort_by(|a, b| a.0.cmp(&b.0));
  return output;
}

pub fn get_lowest(ranges: Vec<(i64, i64)>) -> i64 {
  let mut lowest = -1;

  for range in ranges {
    if lowest == -1 || range.0 < lowest {
      lowest = range.0;
    }
  }

  return lowest;
}
