use std::fs;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
  id: String,
  left: String,
  right: String,
}

impl Node {
  fn new(id: String, left: String, right: String) -> Node {
    Node { id, left, right }
  }
}

fn main() {
  let contents = match fs::read_to_string("src/input.txt") {
    Ok(contents) => contents,
    Err(_) => panic!("Error reading file"),
  };

  let rows = contents.lines();
  let mut steps: Vec<char> = Vec::new();
  let mut nodes : Vec<Node> = Vec::new();

  for (i, row) in rows.enumerate() {
    if i == 0 {
      // "LLR" -> instructions = ['L', 'L', 'R']
      for ch in row.chars() {
        steps.push(ch);
      }
      continue;
    }

    if i == 1 {
      continue;
    }

    // "AAA = (BBB, CCC)" -> current = "AAA"
    let start = match row.split(" = ").nth(0) {
      Some(start) => start.to_string(),
      None => panic!("Error parsing start at i, row {} {}", i, row),
    };

    // "AAA = (BBB, CCC)" -> directions = "(BBB, CCC)"
    let directions = match row.split(" = ").nth(1) {
      Some(directions) => directions
        .trim_matches(|c| c == '(' || c == ')')
        .to_string(),
      None => panic!("Error parsing directions at i, row {} {}", i, row),
    };

    // "BBB, CCC" -> start = "BBB"
    let left = match directions.split(", ").nth(0) {
      Some(start) => start.to_string(),
      None => panic!("Error parsing start at i, row {} {}", i, row),
    };

    // "BBB, CCC" -> end = "CCC"
    let right = match directions.split(", ").nth(1) {
      Some(end) => end.to_string(),
      None => panic!("Error parsing end at i, row {} {}", i, row),
    };

    let node = Node::new(start, left, right);
    nodes.push(node);
  }

  println!("Nodes: {:#?}", nodes);

  let mut total = 0;
  let mut current = "AAA".to_string();

  while current != "ZZZ" {
    total += 1;

    if total % 100_000 == 0 {
      println!("Total: {}", total);
    }

    let step = match steps.get(0) {
      Some(ins) => ins.clone(),
      None => panic!("Error getting instruction"),
    };

    match nodes.iter().find(|node| node.id == current) {
      Some(node) => match step {
        'L' => current = node.left.clone(),
        'R' => current = node.right.clone(),
        _ => panic!("Error parsing instruction: {}", step),
      },
      None => panic!("Error finding node with current {}", current),
    }

    let first = steps.remove(0);
    steps.push(first);
  }

  println!("Total: {}", total);
}
