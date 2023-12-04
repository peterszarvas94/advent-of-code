use std::fs;

#[allow(dead_code)]
struct Number {
  value: usize,
  x: usize,
  y: usize,
}

impl Number {
  fn new(value: usize, x: usize, y: usize) -> Number {
    Number { value, x, y }
  }
}

#[allow(dead_code)]
struct Symbol {
  value: char,
  x: usize,
  y: usize,
}

impl Symbol {
  fn new(value: char, x: usize, y: usize) -> Symbol {
    Symbol { value, x, y }
  }
}

fn is_symbol(c: &char) -> bool {
  match c {
    '.' | '0'..='9' => false,
    _ => true,
  }
}

fn are_adjecent(number: &Number, symbol: &Symbol) -> bool {
  // adjecent:
  // ...#...
  // ..123..
  // .......
  //
  // or
  // ..#....
  // ...123.
  // .......
  //
  // or
  // .......
  // ..123#.
  // .......

  let start_y = match number.y.checked_sub(1) {
    Some(y) => y,
    None => 0,
  };

  let start_x = match number.x.checked_sub(1) {
    Some(x) => x,
    None => 0,
  };

  let adj_y = start_y <= symbol.y && symbol.y <= number.y + 1;
  let adj_x = start_x <= symbol.x && symbol.x <= number.x + number.value.to_string().len();

  adj_y && adj_x
}

fn main() {
  let contents = match fs::read_to_string("src/input.txt") {
    Ok(contents) => contents,
    Err(_) => panic!("Error reading file"),
  };

  let rows = contents.lines();
  let mut numbers: Vec<Number> = Vec::new();
  let mut symbols: Vec<Symbol> = Vec::new();
  let mut total = 0;

  // let mut numbers: Vec<Number> = Vec::new();
  for (y, row) in rows.enumerate() {
    let mut current_number: usize = 0;
    for (x, ch) in row.chars().enumerate() {
      // create numbers
      if let Some(digit) = ch.to_digit(10) {
        current_number = current_number * 10 + digit as usize;

        if x == row.len() - 1 && current_number > 0 {
          let adjusted_x = x - current_number.to_string().len() + 1;
          numbers.push(Number::new(current_number, adjusted_x, y));
        }
      } else if current_number > 0 {
        let adjusted_x = x - current_number.to_string().len();
        numbers.push(Number::new(current_number, adjusted_x, y));
        current_number = 0;
      }

      // check for symbols
      if is_symbol(&ch) {
        symbols.push(Symbol::new(ch, x, y));
      }
    }
  }

  for symbol in symbols {
    let mut adj_numbers: Vec<Number> = Vec::new();

    for number in numbers.iter() {
      if are_adjecent(number, &symbol) {
        adj_numbers.push(Number::new(number.value, number.x, number.y));
      }
    }

    if adj_numbers.len() == 2 {
      let ratio = adj_numbers[0].value * adj_numbers[1].value;
      total += ratio;
    }
  }

  println!("Total: {}", total);
}
