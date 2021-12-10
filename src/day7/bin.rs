use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Sub;


fn main() {
    println!("part1={}", part1());
    println!("part2={}", part2());
}

fn part2() -> u32 {
    let nbs = lines()
        .flat_map(|s| s.split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mean = (nbs.iter().sum::<u32>() as f32 / (nbs.len() as f32)) as u32;
    let sum = nbs.iter()
        .map(|nb| {
            let n = nb.max(&mean).sub(nb.min(&mean));
            n * (n + 1) / 2
        })
        .sum::<u32>();
    sum
}

fn part1() -> u32 {
    let mut nbs = lines()
        .flat_map(|s| s.split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    nbs.sort_unstable();
    let med = match nbs.len() {
        0 => panic!(),
        len if len % 2 == 1 => nbs[len / 2],
        len => (nbs[len / 2] + nbs[(len / 2) - 1]) / 2,
    };
    let sum = nbs.iter()
        .map(|nb| nb.max(&med).sub(nb.min(&med)))
        .sum::<u32>();
    sum
}

fn lines() -> impl Iterator<Item=String> {
    BufReader::new(File::open("src/day7/input.txt").unwrap()).lines()
        .map(Result::unwrap)
}