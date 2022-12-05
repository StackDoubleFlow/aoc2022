use std::fs;

struct Move {
    amt: u32,
    from: u32,
    to: u32,
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut cols = Vec::new();
    
    let mut lines = input.lines();
    for line in lines.by_ref() {
        let line = line.trim_end();
        if line.starts_with(" 1") {
            break;
        }

        let mut i = 0;
        while let Some(c) = line.chars().nth(i * 4 + 1) {
            if c == ' ' {
                i += 1;
                continue;
            }
            if cols.len() <= i {
                cols.extend(std::iter::repeat(Vec::new()).take(i - cols.len() + 1));
            }
            cols[i].push(c);
            i += 1
        }
    }
    lines.next().unwrap();

    cols.iter_mut().for_each(|col| col.reverse());

    for line in lines {
        let words = line.split_whitespace().collect::<Vec<_>>();
        let amt: usize = words[1].parse().unwrap();
        let from: usize = words[3].parse().unwrap();
        let to: usize = words[5].parse().unwrap();

        let from = &mut cols[from - 1];
        let set: Vec<_> = from.drain(from.len() - amt..from.len()).collect();
        cols[to - 1].extend_from_slice(&set);
    }


    for col in cols {
        print!("{}", col.last().unwrap());
    }
    println!();

}
