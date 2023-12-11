use std::fs;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
  id: String,
  left: String,
  right: String,
  steps: usize,
}

impl Node {
  fn new(id: String, left: String, right: String) -> Node {
    Node {
      id,
      left,
      right,
      steps: 0,
    }
  }
}

fn gcd(a: usize, b: usize) -> usize {
  if b == 0 {
    return a;
  }
  return gcd(b, a % b)
}

fn lcm(a: usize, b: usize) -> usize {
  if a == 0 || b == 0 {
    return 0;
  }
  return a * b / gcd(a, b)
}

fn main() {
  let contents = match fs::read_to_string("src/input.txt") {
    Ok(contents) => contents,
    Err(_) => panic!("Error reading file"),
  };

  let rows = contents.lines();
  let mut steps: Vec<char> = Vec::new();
  let mut nodes: Vec<Node> = Vec::new();
  let mut a_nodes: Vec<Node> = Vec::new();

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
    let id = match row.split(" = ").nth(0) {
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

    let node = Node::new(id.clone(), left.clone(), right.clone());
    nodes.push(node);

    let id_last_char = match id.chars().last() {
      Some(ch) => ch,
      None => panic!("Error getting last char of id"),
    };

    // skip if not __A
    if id_last_char == 'A' {
      let a_node = Node::new(id, left, right);
      a_nodes.push(a_node);
    }
  }

  // solve for every A node
  for a_node in a_nodes.iter_mut() {
    // last = 'A'
    let mut last = match a_node.id.chars().last() {
      Some(ch) => ch,
      None => panic!("Error getting last char of id"),
    };

    // current = "AAA"
    let mut current = a_node.id.clone();

    // steps_clone = ['L', 'L', 'R']
    let mut steps_clone = steps.clone();

    // start solving loop
    while last != 'Z' {
      a_node.steps += 1;

      let step = match steps_clone.get(0) {
        Some(ins) => ins.clone(),
        None => panic!("Error getting instruction"),
      };

      match nodes.iter().find(|node| node.id == current) {
        Some(node) => match step {
          'L' => {
            last = match node.left.chars().last() {
              Some(ch) => ch,
              None => panic!("Error getting last char of L"),
            };
            current = node.left.clone();
          }
          'R' => {
            last = match node.right.chars().last() {
              Some(ch) => ch,
              None => panic!("Error getting last char of R"),
            };
            current = node.right.clone();
          }
          _ => panic!("Error parsing instruction at {}", current),
        },
        None => panic!("Error finding node with current {}", current),
      }

      // rotate steps ['L', 'L', 'R'] -> ['L', 'R', 'L'] -> ['R', 'L', 'L']
      let first = steps_clone.remove(0);
      steps_clone.push(first);
    }
  }

  let res = a_nodes.iter().fold(1, |acc, node| lcm(acc, node.steps));
  println!("Result: {}", res);
}
