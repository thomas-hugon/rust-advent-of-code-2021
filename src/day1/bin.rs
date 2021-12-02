use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("part1={}", part1());
    println!("part2={}", part2());
}

fn inc_on_greater((count, last_val): (i32, u32), val: u32) -> (i32, u32) {
    if val > last_val { (count + 1, val) } else { (count, val) }
}

fn part1() -> u32 {
    let (count, _) = BufReader::new(File::open("src/day1/input.txt").unwrap()).lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .fold((-1, 0), inc_on_greater);
    count.max(0) as u32
}

fn part2() -> u32 {
    let vec: Vec<_> = BufReader::new(File::open("src/day1/input.txt").unwrap()).lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap()).collect();
    let (count, _) = vec.windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .fold((-1, 0), inc_on_greater);
    count.max(0) as u32
}