use std::fs;

fn item_priority(item: char) -> u32 {
    let code = item as u32;
    match item {
        'a'..='z' => code - ('a' as u32) + 1,
        'A'..='Z' => code - ('A' as u32) + 27,
        _ => unreachable!()
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let mut sum = 0;
    for elves in lines.chunks_exact(3) {
        let item = ('a'..='z').chain('A'..='Z').find(|c| elves.iter().all(|str| str.contains(*c))).unwrap();
        sum += item_priority(item);
    }

    println!("Sum: {sum}");

}
