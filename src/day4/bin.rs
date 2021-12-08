use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let lines: Vec<_> = lines().collect();
    let numbers: Vec<u8> = lines.get(0).unwrap().split(",").map(|s| s.trim().parse().unwrap()).collect();
    let boards: Vec<_> = lines.iter()
        .filter(|l| !l.is_empty())
        .map(|s| s.as_str())
        .skip(1)
        .collect();
    let mut boards: Vec<Board> = boards.chunks(5).map(|w| Board::from(w)).collect();

    let mut winners: Vec<_> = Vec::with_capacity(boards.len());

    for &nb in numbers.iter() {
        let current_winners_scores: Vec<_> = boards.iter_mut()
            .filter(|b| matches!(b.score(), None))
            .filter_map(|b| b.mark(nb))
            .collect();

        winners.extend(current_winners_scores);
    }

    println!("part1={:?}", winners.first().unwrap());
    println!("part2={:?}", winners.last().unwrap());
}

#[derive(Debug)]
enum BoardNumber {
    Marked,
    UnMarked(u8),
}

#[derive(Debug)]
struct Board {
    board: [[BoardNumber; 5]; 5],
    score: Option<u32>,
}

impl Board {
    fn score(&self) -> Option<u32> {
        self.score
    }
    fn mark(&mut self, nb: u8) -> Option<u32> {
        let found = if let Some((i, j, board)) = self.board.iter_mut().enumerate()
            .flat_map(|(i, bs)| bs.iter_mut().enumerate().map(move |(j, b)| (i, j, b)))
            .find(|(_, _, b)| matches!(b, BoardNumber::UnMarked(unb) if *unb == nb)) {
            *board = BoardNumber::Marked;
            Some((i, j))
        } else {
            None
        };

        self.score = if found.map_or(false, |coord| self.compute_winner(coord)) {
            Some(self.compute_score(nb as u32))
        } else {
            None
        };

        self.score
    }

    fn compute_score(&self, nb: u32) -> u32 {
        let score: u32 = self.board.iter()
            .flat_map(|bs| bs.iter())
            .map(|b| match b {
                BoardNumber::Marked => 0,
                BoardNumber::UnMarked(x) => *x as u32,
            })
            .sum();
        nb * score
    }

    fn compute_winner(&self, (i, j): (usize, usize)) -> bool {
        (0..5usize).all(|j| matches!(&self.board[i][j], BoardNumber::Marked))
            || (0..5usize).all(|i| matches!(&self.board[i][j], BoardNumber::Marked))
    }
}

impl From<&[&str]> for Board {
    fn from(input: &[&str]) -> Self {
        Board {
            board: input.iter()
                .map(|s| s.split_whitespace().filter(|s| !s.is_empty()).map(|s| BoardNumber::UnMarked(s.parse().unwrap())).collect::<Vec<_>>().try_into().expect("fuck"))
                .collect::<Vec<_>>().try_into().expect("fuckfuck"),
            score: None,
        }
    }
}


fn lines() -> impl Iterator<Item=String> {
    BufReader::new(File::open("src/day4/input.txt").unwrap()).lines()
        .map(Result::unwrap)
}