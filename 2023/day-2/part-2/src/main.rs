use std::fs;

struct Min {
  red: usize,
  green: usize,
  blue: usize,
}

impl Min {
  fn new(red: usize, green: usize, blue: usize) -> Min {
    Min { red, green, blue }
  }

  fn set_color(&mut self, color: &str, amount: usize) {
    match color {
      "red" => self.red = amount,
      "green" => self.green = amount,
      "blue" => self.blue = amount,
      _ => panic!("Error parsing color"),
    }
  }
}

fn main() {
  let contents = match fs::read_to_string("src/input.txt") {
    Ok(contents) => contents,
    Err(_) => panic!("Error reading file"),
  };

  let rows = contents.lines();

  let mut total = 0;

  for row in rows {
    println!("row: {}", row);


    let mut min = Min::new(0, 0, 0);
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
            if amount > min.red {
              min.set_color("red", amount);
            }
          }
          "green" => {
            if amount > min.green {
              min.set_color("green", amount);
            }
          }
          "blue" => {
            if amount > min.blue {
              min.set_color("blue", amount);
            }
          }
          _ => panic!("Error parsing color"),
        }
      }
    }
    
    let power = min.red * min.blue * min.green;
    total += power;
  }

  println!("Total: {}", total);
}
