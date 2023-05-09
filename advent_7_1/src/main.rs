use std::fs::read_to_string;

struct Dir {
    path: String,
    size: u64,
}

impl Dir {
    fn new(path: &str, size: u64) -> Dir {
        Dir {
            path: String::from(path),
            size,
        }
    }

    fn add_to_size(&mut self, size: u64) {
        self.size += size;
    }
}

fn get_sub_paths(path: &str) -> Vec<String> {
    let folders = path.split("/").collect::<Vec<&str>>();
    let mut paths: Vec<String> = Vec::new();
    for (i, _) in folders.iter().enumerate() {
        let path = folders[0..=i].join("/");
        paths.push(path);
    }
    return paths;
}

fn main() {
    // open file
    let file = match read_to_string("src/input") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut dirs: Vec<Dir> = Vec::new();
    let mut current_path = String::from("root");

    let lines = file.lines().enumerate();
    for (i, line) in lines {
        // skip root
        if i == 0 {
            continue;
        }

        // go to parent directory
        if line.starts_with("$ cd ..") {
            let mut path = current_path.split("/").collect::<Vec<&str>>();
            path.pop();
            current_path = path.join("/");
            continue;
        }

        // go to child directory
        if line.starts_with("$ cd") {
            let folder_name = line.split(" ").collect::<Vec<&str>>()[2];
            current_path = format!("{}/{}", current_path, folder_name);

            // check if exists, create if not
            let is_exists = dirs.iter().any(|dir| dir.path == current_path);
            if !is_exists {
                let dir = Dir::new(&current_path, 0);
                dirs.push(dir);
            }
        }

        // read file size
        let props = line.split(" ").collect::<Vec<&str>>();
        let size = match props[0].parse::<u64>() {
            Ok(s) => s,
            Err(_) => continue,
        };

        // add to directories sizes
        let paths = get_sub_paths(&current_path);
        for path in paths {
            let dir = match dirs.iter_mut().find(|dir| dir.path == path) {
                Some(d) => d,
                None => continue,
            };
            dir.add_to_size(size);
        }
    }

    // get full size of directories, where size less than 100000
    let total = dirs
        .iter()
        .filter(|dir| dir.size <= 100000)
        .fold(0, |acc, dir| acc + dir.size);

    println!("Total: {}", total);
}
