use std::{cmp::Ordering, collections::HashMap, fs};

#[derive(Debug)]
enum Combination {
  FiveOfKind = 6,
  FourOfKind = 5,
  FullHouse = 4,
  ThreeOfKind = 3,
  TwoPairs = 2,
  OnePair = 1,
  HighCard = 0,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Card {
  Ace = 14,
  King = 13,
  Queen = 12,
  Jack = 11,
  Ten = 10,
  Nine = 9,
  Eight = 8,
  Seven = 7,
  Six = 6,
  Five = 5,
  Four = 4,
  Three = 3,
  Two = 2,
}

impl Card {
  fn from_char(c: char) -> Option<Card> {
    match c {
      'A' => Some(Card::Ace),
      'K' => Some(Card::King),
      'Q' => Some(Card::Queen),
      'J' => Some(Card::Jack),
      'T' => Some(Card::Ten),
      '9' => Some(Card::Nine),
      '8' => Some(Card::Eight),
      '7' => Some(Card::Seven),
      '6' => Some(Card::Six),
      '5' => Some(Card::Five),
      '4' => Some(Card::Four),
      '3' => Some(Card::Three),
      '2' => Some(Card::Two),
      _ => None,
    }
  }
}

#[derive(Debug)]
struct Hand {
  cards: [Card; 5],
  bid: u64,
}

impl Hand {
  fn from_str(row: &str) -> Option<Hand> {
    let hand_str = match row.split(" ").nth(0) {
      Some(s) => s,
      None => return None,
    };

    let bid = match row.split(" ").nth(1) {
      Some(s) => match s.parse::<u64>() {
        Ok(n) => n,
        Err(_) => return None,
      },
      None => return None,
    };

    let mut cards: [Card; 5] = [Card::Two; 5];
    let mut i = 0;
    for c in hand_str.chars() {
      if let Some(card) = Card::from_char(c) {
        cards[i] = card;
        i += 1;
      }
    }
    if i == 5 {
      Some(Hand { cards, bid })
    } else {
      None
    }
  }

  fn count_cards(&self) -> HashMap<Card, u8> {
    let mut counts_map: HashMap<Card, u8> = HashMap::new();
    for &card in self.cards.iter() {
      *counts_map.entry(card).or_insert(0) += 1;
    }
    return counts_map;
  }

  fn get_combination(&self) -> Combination {
    let counts_map = self.count_cards();

    let mut counts_vec: Vec<(Card, u8)> = counts_map.into_iter().collect();
    counts_vec.sort_by(|a, b| b.1.cmp(&a.1));

    let first = counts_vec.first();
    let second = counts_vec.get(1);

    match first {
      Some((_, 5)) => return Combination::FiveOfKind,
      Some((_, 4)) => return Combination::FourOfKind,
      Some((_, 3)) => match second {
        Some((_, 2)) => return Combination::FullHouse,
        _ => return Combination::ThreeOfKind,
      },
      Some((_, 2)) => match second {
        Some((_, 2)) => return Combination::TwoPairs,
        _ => return Combination::OnePair,
      },
      _ => {}
    }

    return Combination::HighCard;
  }

  fn compare(&self, enemy: &Hand) -> Ordering {
    let my_combo = self.get_combination() as u8;
    let emeny_combo = enemy.get_combination() as u8;

    // first compare combinations
    let compo_ordering = my_combo.cmp(&emeny_combo);
    if compo_ordering != Ordering::Equal {
      return compo_ordering;
    }

    // then compare cards
    for (i, card) in self.cards.iter().enumerate() {
      let my_card = *card as u8;
      let enemy_card = enemy.cards[i] as u8;
      let card_ordering = my_card.cmp(&enemy_card);
      if card_ordering != Ordering::Equal {
        return card_ordering;
      }
    }

    return Ordering::Equal;
  }
}

fn main() {
  // test
  let contents = match fs::read_to_string("src/input.txt") {
    Ok(contents) => contents,
    Err(_) => panic!("Error reading file"),
  };
  // end test

  let rows = contents.lines();
  let mut total = 0;
  let mut hands: Vec<Hand> = Vec::new();

  for row in rows {
    let hand = match Hand::from_str(row) {
      Some(hand) => hand,
      None => {
        println!("Error parsing hand: {}", row);
        continue;
      }
    };
    hands.push(hand);
  }

  hands.sort_by(|a, b| a.compare(b));
  for (i, hand) in hands.iter().enumerate() {
    println!("{}: {:?}", i + 1, hand);
    let factor = i + 1;
    let points = hand.bid * factor as u64;
    total += points;
  }

  println!("Total: {}", total);
}
