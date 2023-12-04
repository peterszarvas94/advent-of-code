use std::fs::read_to_string;

#[derive(Clone, Debug)]
struct Tower {
    crates: Vec<char>,
}

impl Tower {
    fn new() -> Self {
        Self { crates: Vec::new() }
    }
    fn push(&mut self, c: char) {
        self.crates.push(c);
    }
    fn remove(&mut self) -> char {
        self.crates.remove(0)
    }
    fn add(&mut self, c: char) {
        self.crates.insert(0, c);
    }
}

fn move_crate(quantity: usize, from: usize, to: usize, v: &mut [Tower]) {
    for _ in 0..quantity {
        // let item = match v[from].crates.get(0) {
        //     Some(c) => *c,
        //     None => break,
        // };
        let item = v[from].remove();
        v[to].add(item);
    }
}

fn main() {
    // open file
    let file = read_to_string("src/input");
    let file = match file {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut v: Vec<Tower> = vec![Tower::new(); 9];

    // read file line by line
    file.lines().enumerate().for_each(|(i, line)| {
        if i < 8 {
            // put crates into towers
            line.chars().enumerate().for_each(|(j, c)| {
                if (j + 3) % 4 == 0 {
                    let index = (j - 1) / 4;
                    if c != ' ' {
                        v[index].push(c);
                    }
                }
            });
        }

        if i > 9 {
            // collect words from instruction into a vector, and parse them into numbers
            let words: Vec<usize> = line
                .split(" ")
                .map(|s| match s.parse() {
                    Ok(n) => n,
                    Err(_) => 0,
                })
                .collect();

            move_crate(words[1], words[3] - 1, words[5] - 1, &mut v);
        }
    });

    // get all frirst elements of towers, and convert to string
    let result = v.iter().map(|t| t.crates[0]).collect::<String>();

    println!("{}", result);
}
