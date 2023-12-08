mod utils;

use crate::utils::*;
use std::{fs, i64};

fn main() {
  let contents = match fs::read_to_string("src/input.txt") {
    Ok(contents) => contents,
    Err(_) => panic!("Error reading file"),
  };

  let rows = contents.lines();

  let mut maps: Vec<Map> = Vec::new();
  let mut lines_temp: Vec<Line> = Vec::new();
  let mut seeds: Vec<(i64, i64)> = Vec::new();

  let mut empty_row = 0;

  for (i, row) in rows.enumerate() {
    if i == 0 {
      seeds = row
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
        .chunks(2)
        .map(|x| (x[0], x[0] + x[1] - 1))
        .collect::<Vec<(i64, i64)>>();
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
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

      let line = Line {
        target_start: data[0],
        source_start: data[1],
        range: data[2],
      };

      lines_temp.push(line);
    }
  }

  if lines_temp.len() > 0 {
    let map = Map { lines: lines_temp };
    maps.push(map);
  }

  let mut ranges = seeds;

  println!("start ranges: {:?}", ranges);
  for map in maps {
    println!("new map");
    let new_ranges = ranges_map(ranges, &map);
    println!("- new ranges: {:?}", new_ranges);
    ranges = new_ranges;
    println!("lowest 1: {}", get_lowest(ranges.clone()));
  }

  let lowest = get_lowest(ranges);
  println!("lowest: {}", lowest);
}
