use std::{fs, collections::HashMap};

#[derive(Debug)]
enum File {
    File(usize),
    Directory(Directory)
}

#[derive(Default, Debug)]
struct Directory {
    files: HashMap<String, File>,
    size: usize,
}

impl Directory {
    fn calculate_size(&mut self) -> usize {
        self.size = self.files.values_mut().map(|file| match file {
            File::File(size) => *size,
            File::Directory(dir) => dir.calculate_size()
        }).sum();
        self.size
    }

    fn sum(&self) -> usize {
        let mut sum = 0;
        if self.size <= 100000 {
            sum += self.size;
        }
        for val in self.files.values() {
            if let File::Directory(dir) = val {
                sum += dir.sum()
            }
        }
        sum
    }

    fn find_smallest<'a>(&'a self, size_list: &mut Vec<(&'a str, usize)>) {
        for (key, val) in &self.files {
            if let File::Directory(dir) = val {
                size_list.push((key, dir.size));
                dir.find_smallest(size_list);
            }
        }
    }
}

fn main() {
    let mut cur_path = Vec::new();
    let mut root_dir: Directory = Default::default();

    let input = fs::read_to_string("./input.txt").unwrap();
    let mut lines = input.lines().peekable();
    lines.next().unwrap();
    while let Some(command) = lines.next() {
        let mut command = command.trim_start_matches("$ ").split_whitespace();
        match command.next().unwrap() {
            "cd" => {
                let to = command.next().unwrap();
                if to == ".." {
                    cur_path.pop();
                } else {
                    cur_path.push(to.to_string());
                }
            }
            "ls" => {
                let mut cur_dir = &mut root_dir;
                
                for p in &cur_path {
                    let entry = cur_dir.files.entry(p.clone()).or_insert_with(|| File::Directory(Default::default()));
                    match entry {
                        File::Directory(dir) => cur_dir = dir,
                        _ => panic!(),
                    };
                }

                while let Some(file) = lines.next() {
                    if let Some(dir) = file.strip_prefix("dir ") {
                        cur_dir.files.insert(dir.to_string(), File::Directory(Default::default()));
                    } else {
                        let mut entry = file.split_whitespace();
                        let size = entry.next().unwrap();
                        let name = entry.next().unwrap();
                        cur_dir.files.insert(name.to_string(), File::File(str::parse(size).unwrap()));
                    }

                    match lines.peek() {
                        Some(s) if s.starts_with('$') => break,
                        _ => {}
                    }
                }
            }
            _ => panic!(),
        }
    }

    let root_size = root_dir.calculate_size();

    let mut size_list = Vec::new();
    root_dir.find_smallest(&mut size_list);
    size_list.sort_by_key(|(_, x)| *x);
    size_list.retain(|(_, x)| *x > 30000000 - (70000000 - root_size));

    println!("The sum is {}", root_dir.sum());
    println!("The smallest size is {}", size_list[0].1);
}
