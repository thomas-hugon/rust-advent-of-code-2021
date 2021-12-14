use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::Line::Valid;

fn main() {
    let lines = lines()
        .map(|line| decode(&line))
        .collect::<Vec<_>>();

    let corrupted_score = lines.iter()
        .map(score_corrupted)
        .sum::<u32>();

    let mut incomplete_scores = lines.iter()
        .filter_map(score_incomplete)
        .collect::<Vec<_>>();
    incomplete_scores.sort_unstable();
    let incomplete_total_score = incomplete_scores[incomplete_scores.len() / 2];

    println!("part1={}", corrupted_score);
    println!("part2={}", incomplete_total_score);
}

fn score_corrupted(line: &Line) -> u32 {
    match line {
        Line::Corrupted(')') => 3,
        Line::Corrupted(']') => 57,
        Line::Corrupted('}') => 1197,
        Line::Corrupted('>') => 25137,
        _ => 0
    }
}

fn score_incomplete(line: &Line) -> Option<u64> {
    fn score_incomplete(remainings: &Vec<char>) -> u64 {
        remainings.iter().rev().fold(0, |score, c| score * 5 + match c {
            &'(' => 1,
            &'[' => 2,
            &'{' => 3,
            &'<' => 4,
            _ => panic!()
        })
    }
    match line {
        Line::Incomplete(x) => Some(score_incomplete(x)),
        _ => None
    }
}

#[derive(Debug)]
enum Line {
    Valid,
    Incomplete(Vec<char>),
    Corrupted(char),
}

fn decode(line: &str) -> Line {
    match line.chars()
        .fold(Line::Incomplete(vec![]), |mut stack, c| {
            match (&mut stack, c) {
                (Line::Incomplete(ref mut stack), val @ ('(' | '[' | '{' | '<')) => stack.push(val),
                (Line::Incomplete(ref mut stack), ')') if matches!(stack.last(), Some(&'(')) => { stack.pop(); }
                (Line::Incomplete(ref mut stack), ']') if matches!(stack.last(), Some(&'[')) => { stack.pop(); }
                (Line::Incomplete(ref mut stack), '}') if matches!(stack.last(), Some(&'{')) => { stack.pop(); }
                (Line::Incomplete(ref mut stack), '>') if matches!(stack.last(), Some(&'<')) => { stack.pop(); }
                (Line::Incomplete(_), invalid_char @ (')' | ']' | '}' | '>')) => return Line::Corrupted(invalid_char),
                (Line::Corrupted(_), _) => {}
                _ => panic!()
            }
            stack
        }){
        c@Line::Corrupted(_) => c,
        Line::Incomplete(vec) if vec.is_empty()=> Valid,
        i@Line::Incomplete(_) => i,
        v@Line::Valid => v
    }
}

fn lines() -> impl Iterator<Item=String> {
    BufReader::new(File::open("src/day10/input.txt").unwrap()).lines()
        .map(Result::unwrap)
}