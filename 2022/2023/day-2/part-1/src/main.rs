use std::fs;

struct Max {
  red: usize,
  green: usize,
  blue: usize,
}

impl Max {
  fn new(red: usize, green: usize, blue: usize) -> Max {
    Max { red, green, blue }
  }
}

fn main() {
  let contents = match fs::read_to_string("src/input.txt") {
    Ok(contents) => contents,
    Err(_) => panic!("Error reading file"),
  };

  let rows = contents.lines();

  let max = Max::new(12, 13, 14);
  let mut total = 0;

  for row in rows {
    let mut possible = true;

    println!("row: {}", row);
    // 'Game 1: 3 blue, 4 green, 5 red; 3 red, 4 green, 5 blue
    // game_name = 'Game 1'
    let game_name = match row.split(": ").nth(0) {
      Some(name) => name,
      None => panic!("Error getting game id at row {}", row),
    };

    // game_id_ch = '1'
    let game_id_ch = match game_name.split(" ").nth(1) {
      Some(id) => id,
      None => panic!("Error getting game id at row {}", row),
    };

    // game_id = 1
    let game_id = match game_id_ch.parse::<usize>() {
      Ok(id) => id,
      Err(_) => panic!("Error parsing game id at row {}", row),
    };

    // round_list = '3 blue, 4 green, 5 red; 3 red, 4 green, 5 blue'
    let round_list = match row.split(": ").nth(1) {
      Some(r) => r,
      None => panic!("Error getting rounds at row {}", row),
    };

    // rounds = ['3 blue, 4 green, 5 red', '3 red, 4 green, 5 blue']
    let rounds = round_list.split("; ");

    for round in rounds {
      // round = '3 blue, 4 green, 5 red'
      println!(" round: {}", round);

      // picks = ['3 blue', '4 green', '5 red']
      let picks = round.split(", ");

      for pick in picks {
        // pick = '3 blue'

        println!("  pick: {}", pick);

        // compare against max
        let color = match pick.split(" ").nth(1) {
          Some(c) => c,
          None => panic!("Error getting color at row {}", row),
        };
        println!("   color: {}", color);

        let amount = match pick.split(" ").nth(0).unwrap().parse::<usize>() {
          Ok(a) => a,
          Err(_) => panic!("Error parsing amount at row {}", row),
        };
        println!("   amount: {}", amount);

        match color {
          "red" => {
            if amount > max.red {
              possible = false;
            }
          }
          "green" => {
            if amount > max.green {
              possible = false;
            }
          }
          "blue" => {
            if amount > max.blue {
              possible = false;
            }
          }
          _ => panic!("Error parsing color at row {}", row),
        }
      }
    }
    println!("possible: {}", possible);
    if possible {
      println!("adding: {}", game_id);
      total += game_id;
    }
  }

  println!("Total: {}", total);
}
