use std::ops::Range;
use std::fs;

#[derive(Debug)]
struct Pair(Range<u32>, Range<u32>);

fn parse_range(str: &str) -> Range<u32> {
    let mut nums = str.split('-').map(str::parse);
    let start = nums.next().unwrap().unwrap();
    let end = nums.next().unwrap().unwrap();
    Range {
        start,
        end
    }

}

impl Pair {
    fn parse(str: &str) -> Pair {
        let mut ranges = str.split(',').map(parse_range);
        Pair(
            ranges.next().unwrap(),
            ranges.next().unwrap()
        )
    }
}

fn range_overlaps(a: Range<u32>, b: Range<u32>) -> bool {
    a.start <= b.start && b.start <= a.end
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut count = 0;
    for line in input.lines() {
        let pair = Pair::parse(line);
        if range_overlaps(pair.0.clone(), pair.1.clone()) || range_overlaps(pair.1, pair.0) {
            count += 1;
        }
    }
    println!("Count: {count}");
}
