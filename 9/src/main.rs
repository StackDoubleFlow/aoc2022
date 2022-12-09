use std::fs;
use std::collections::HashSet;

fn move_pos(pos: &mut (i32, i32), dir: char) {
    match dir {
        'U' => pos.0 += 1,
        'D' => pos.0 -= 1,
        'L' => pos.1 -= 1,
        'R' => pos.1 += 1,
        _ => panic!(),
    }
}
fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut visited = HashSet::new();

    let mut knots = vec![(0, 0); 10];
    for line in input.lines() {
        let dir = line.chars().next().unwrap();
        let amt = str::parse(&line[2..]).unwrap();
        for _ in 0..amt {
            move_pos(&mut knots[0], dir);
            for i in 0..knots.len() - 1 {
                let head_pos = knots[i];
                let mut tail_pos = knots[i + 1];
                let drow: i32 = head_pos.0 - tail_pos.0;
                let dcol: i32 = head_pos.1 - tail_pos.1;
                let touching = drow.abs() <= 1 && dcol.abs() <= 1;
                if !touching {
                    tail_pos.0 += drow.signum();
                    tail_pos.1 += dcol.signum();
                }
                knots[i + 1] = tail_pos;
            }
            visited.insert(*knots.last().unwrap());
        }
    }

    dbg!(visited.len());
}
