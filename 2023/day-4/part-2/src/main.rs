use std::fs;

#[derive(Debug)]
struct Card {
  id: usize,
  copies: usize,
}

impl Card {
  fn new(id: usize, copies: usize) -> Card {
    Card { id, copies }
  }
}

#[derive(Debug)]
struct Deck {
  cards: Vec<Card>,
}

impl Deck {
  fn new() -> Deck {
    Deck { cards: Vec::new() }
  }

  fn add_copy(&mut self, id: usize, copies: usize) {
    let mut found = false;

    for card in self.cards.iter_mut() {
      if card.id == id {
        card.copies += copies;
        found = true;
        break;
      }
    }

    if !found {
      let card = Card::new(id, 1);
      self.cards.push(card);
    }
  }

  fn get_copy_number(&self, id: usize) -> usize {
    for card in self.cards.iter() {
      if card.id == id {
        return card.copies;
      }
    }

    0
  }
}

fn main() {
  let contents = match fs::read_to_string("src/input.txt") {
    Ok(contents) => contents,
    Err(_) => panic!("Error reading file"),
  };

  let rows = contents.lines();
  let mut total = 0;

  let mut deck = Deck::new();

  let count = rows.clone().count();
  for i in 0..count {
    deck.add_copy(i + 1, 1);
  }

  for card in deck.cards.iter() {
    println!(" {:?}", card);
  }

  for row in rows {
    let mut points = 0;

    let game = row.split(": ").collect::<Vec<&str>>();

    let id = match game[0].split_whitespace().collect::<Vec<&str>>()[1].parse::<usize>() {
      Ok(id) => id,
      Err(_) => panic!("Error parsing id"),
    };

    let numbers = game[1].split(" | ").collect::<Vec<&str>>();

    let winning = numbers[0]
      .split_whitespace()
      .filter_map(|n| n.parse::<i32>().ok())
      .collect::<Vec<i32>>();

    let my = numbers[1]
      .split_whitespace()
      .filter_map(|n| n.parse::<i32>().ok())
      .collect::<Vec<i32>>();

    for n in my.iter() {
      if winning.contains(n) {
        points += 1;
      }
    }

    let copy_number = deck.get_copy_number(id);
    println!("id {} copy {} points {}", id, copy_number, points);

    for i in 1..=points {
      deck.add_copy(id + i, copy_number);
    }

    for card in deck.cards.iter() {
      println!(" {:?}", card);
    }
  }

  for card in deck.cards.iter() {
    total += card.copies;
  }

  println!("Total: {}", total);
}
