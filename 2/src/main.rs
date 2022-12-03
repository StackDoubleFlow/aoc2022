use std::fs;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_ch(c: char) -> Self {
        match c {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            _ => unreachable!()
        }
    }

    fn winning_response(self) -> Self {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn losing_response(self) -> Self {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    fn score(self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

enum Outcome {
    Win,
    Lose,
    Draw
}

impl Outcome {
    fn from_ch(c: char) -> Self {
        match c {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => unreachable!()
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let moves = input.lines().map(|line| {
        let mut words = line.split_whitespace();
        let opponent = Move::from_ch(words.next().unwrap().chars().next().unwrap());
        let response = Outcome::from_ch(words.next().unwrap().chars().next().unwrap());
        (opponent, response)
    });

    let mut score = 0;
    for (opponent, outcome) in moves {
        let response = match outcome {
            Outcome::Lose => opponent.losing_response(),
            Outcome::Draw => {
                score += 3;
                opponent
            }
            Outcome::Win => {
                score += 6;
                opponent.winning_response()
            }
        };
        score += response.score();
    }

    println!("The final score is {}", score);
}
