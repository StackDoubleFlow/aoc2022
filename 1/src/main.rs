use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();

    let mut cur_elf = 0;
    let mut all_elves = Vec::new();
    for line in file.lines() {
        if line.is_empty() {
            all_elves.push(cur_elf);
            cur_elf = 0;
            continue;
        }

        cur_elf += line.parse::<u32>().unwrap();
    }

    all_elves.sort();
    all_elves.reverse();


    for (i, x) in all_elves.iter().enumerate().take(3) {
        println!("elf {i} with {x} calories", i = i + 1);
    }
}
