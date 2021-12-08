use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
}

fn main() {
    let part1 = lines()
        .map(line_def)
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
        .flat_map(points)
        .fold(HashMap::new(), |mut acc, value| {
            *acc.entry(value).or_insert(0) += 1;
            acc
        })
        .values()
        .filter(|nb| **nb > 1)
        .count();

    let part2 = lines()
        .map(line_def)
        .flat_map(points)
        .fold(HashMap::new(), |mut acc, value| {
            *acc.entry(value).or_insert(0) += 1;
            acc
        })
        .values()
        .filter(|nb| **nb > 1)
        .count();

    println!("part1={}", part1);
    println!("part2={}", part2);
}

fn line_def(line_def: String) -> ((u16, u16), (u16, u16)) {
    let captures = RE.captures(line_def.as_str()).unwrap();
    ((captures.get(1).unwrap().as_str().parse().unwrap(), captures.get(2).unwrap().as_str().parse().unwrap()),
     (captures.get(3).unwrap().as_str().parse().unwrap(), captures.get(4).unwrap().as_str().parse().unwrap())
    )
}

fn points(((x1, y1), (x2, y2)): ((u16, u16), (u16, u16))) -> Box<dyn Iterator<Item=(u16, u16)>> {
    if x1 == x2 || y1 == y2 {
        Box::new((x1.min(x2)..=x2.max(x1))
            .flat_map(move |x| (y1.min(y2)..=y2.max(y1)).map(move |y| (x, y)))
        )
    } else {
        Box::new(iter_inc(x1, x2).zip(iter_inc(y1, y2)))
    }
}

fn iter_inc(x1: u16, x2: u16) -> Box<dyn Iterator<Item=u16>> {
    if x1 < x2 { Box::new(x1..=x2) } else { Box::new((x2..=x1).rev()) }
}

fn lines() -> impl Iterator<Item=String> {
    BufReader::new(File::open("src/day5/input.txt").unwrap()).lines()
        .map(Result::unwrap)
}