use std::fs;
use take_until::TakeUntilExt;

fn check_line<'a>(mut line: impl Iterator<Item = &'a mut (u32, bool)>) {
    let (max_height, first) = line.next().unwrap();
    *first = true;
    let mut max_height = *max_height;
    for (height, visible) in line {
        if *height > max_height {
            *visible = true;
        }
        max_height = max_height.max(*height);
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut trees: Vec<Vec<(u32, bool)>> = input.lines().map(|line| {
        line.chars().map(|c| (c as u32 - '0' as u32, false)).collect()
    }).collect();

    trees.iter_mut().for_each(|row| check_line(row.iter_mut()));
    trees.iter_mut().for_each(|row| check_line(row.iter_mut().rev()));

    (0..trees.len()).for_each(|col_idx| check_line(trees.iter_mut().map(|row| &mut row[col_idx])));
    (0..trees.len()).for_each(|col_idx| check_line(trees.iter_mut().rev().map(|row| &mut row[col_idx])));

    let count: usize = trees.iter().map(|row| row.iter().filter(|(_, vis)| *vis).count()).sum();
    dbg!(count);

    let mut best_score = 0;
    let mut best_tree = (0, 0);
    for row in 0..trees.len() {
        for col in 0..trees[0].len() {
            let height = trees[row][col].0;
            let up = (0..row).rev().take_until(|row| trees[*row][col].0 >= height).count();
            let down = (row + 1..trees.len()).take_until(|row| trees[*row][col].0 >= height).count();
            let left = (0..col).rev().take_until(|col| trees[row][*col].0 >= height).count();
            let right = (col + 1..trees[0].len()).take_until(|col| trees[row][*col].0 >= height).count();
            let score = up * down * left * right;
            if score > best_score {
                best_score = score;
                best_tree = (row, col);
            }
        }
    }
    dbg!(best_tree);
    dbg!(best_score);
}
