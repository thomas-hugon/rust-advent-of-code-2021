use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("part1={}", part1());
    println!("part2={}", part2());
}

fn part1() -> u32 {
    lines()
        .flat_map(|s| s.split(" | ").last().unwrap().split(" ").map(|s| String::from(s)).collect::<Vec<_>>())
        .filter(|s| matches!(s.len(), 2|3|4|7))
        .count() as u32
}

fn part2() -> u32 {
    lines()
        .map(solve)
        .sum()
}

fn solve(input: String) -> u32 {
    let (digits, output) = input.split_once(" | ").map(|(s1, s2)| (
        s1.split_whitespace().collect::<Vec<_>>(),
        s2.split_whitespace().collect::<Vec<_>>()
    )).unwrap();

    let one = digits.iter().filter(|s| s.len() == 2).next().unwrap().chars().collect::<Vec<_>>();
    let seven = digits.iter().filter(|s| s.len() == 3).next().unwrap().chars().collect::<Vec<_>>();
    let four = digits.iter().filter(|s| s.len() == 4).next().unwrap().chars().collect::<Vec<_>>();
    let eight = digits.iter().filter(|s| s.len() == 7).next().unwrap().chars().collect::<Vec<_>>();

    let six = digits.iter().filter(|s| s.len() == 6)
        .filter(|s| !s.contains(one[0]) || !s.contains(one[1]))
        .next().unwrap().chars().collect::<Vec<_>>();
    let nine = digits.iter().filter(|s| s.len() == 6)
        .filter(|s| s.contains(four[0]) && s.contains(four[1]) && s.contains(four[2]) && s.contains(four[3]))
        .next().unwrap().chars().collect::<Vec<_>>();
    let zero = digits.iter().filter(|s| s.len() == 6)
        .filter(|s| s.chars().collect::<Vec<_>>() != six && s.chars().collect::<Vec<_>>() != nine)
        .next().unwrap().chars().collect::<Vec<_>>();
    let three = digits.iter().filter(|s| s.len() == 5)
        .filter(|s| s.contains(one[0]) && s.contains(one[1]))
        .next().unwrap().chars().collect::<Vec<_>>();
    let five = digits.iter().filter(|s| s.len() == 5)
        .filter(|s| {
            let s = s.chars().collect::<Vec<_>>();
            six.contains(&s[0]) && six.contains(&s[1]) && six.contains(&s[2]) && six.contains(&s[3]) && six.contains(&s[4])
        })
        .next().unwrap().chars().collect::<Vec<_>>();

    let two = digits.iter().filter(|s| s.len() == 5)
        .filter(|s| s.chars().collect::<Vec<_>>() != three && s.chars().collect::<Vec<_>>() != five)
        .next().unwrap().chars().collect::<Vec<_>>();


    output.iter().map(|s| match s.chars().collect::<HashSet<_>>() {
        x if x == HashSet::from_iter(zero.clone()) => 0,
        x if x == HashSet::from_iter(one.clone()) => 1,
        x if x == HashSet::from_iter(two.clone()) => 2,
        x if x == HashSet::from_iter(three.clone()) => 3,
        x if x == HashSet::from_iter(four.clone()) => 4,
        x if x == HashSet::from_iter(five.clone()) => 5,
        x if x == HashSet::from_iter(six.clone()) => 6,
        x if x == HashSet::from_iter(seven.clone()) => 7,
        x if x == HashSet::from_iter(eight.clone()) => 8,
        x if x == HashSet::from_iter(nine.clone()) => 9,
        _ => panic!()
    })
        .fold(0, |acc, nb| acc * 10 + nb)
}


fn lines() -> impl Iterator<Item=String> {
    BufReader::new(File::open("src/day8/input.txt").unwrap()).lines()
        .map(Result::unwrap)
}