use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let mut iter = lines();
    let mut dots = HashSet::new();
    let mut folds = Vec::new();

    while let Some((x, y)) = iter.next().and_then(|line| line
        .split_once(",")
        .map(|(s1, s2)| (i32::from_str(s1).unwrap(), i32::from_str(s2).unwrap()))) {
        dots.insert((x, y));
    }
    for line in iter {
        folds.push((line.strip_prefix("fold along x=").map_or(-1, |s| s.parse::<i32>().unwrap()),
                    line.strip_prefix("fold along y=").map_or(-1, |s| s.parse::<i32>().unwrap()))
        )
    }

    println!("part1={}", part1(dots.clone(), &folds));
    println!("part2=\n{}", part2(dots.clone(), &folds));
}

fn part1(mut dots: HashSet<(i32, i32)>, folds: &Vec<(i32, i32)>) -> i32 {
    fold(&mut dots, folds[0]);
    dots.len() as i32
}

fn part2(mut dots: HashSet<(i32, i32)>, folds: &Vec<(i32, i32)>) -> String {
    for curr_fold in folds {
        fold(&mut dots, *curr_fold);
    }

    let (width, height) = dots.iter()
        .fold((0, 0), |(max_x, max_y), (x, y)| (max_x.max(*x), max_y.max(*y)));

    let mut s = String::new();
    for y in 0..=height {
        for x in 0..=width {
            s.push_str(if dots.contains(&(x, y)) { "&" } else { " " });
        }
        s.push_str("\n");
    }
    s
}

fn fold(dots: &mut HashSet<(i32, i32)>, (fold_x, fold_y): (i32, i32)) {
    let folded = dots.iter().filter(|(x, y)| x > &fold_x && y > &fold_y).cloned().collect::<Vec<_>>();
    for (x, y) in folded {
        let new_x = if fold_x > -1 { 2 * fold_x - x } else { x };
        let new_y = if fold_y > -1 { 2 * fold_y - y } else { y };
        dots.remove(&(x, y));
        dots.insert((new_x, new_y));
    }
}

fn lines() -> impl Iterator<Item=String> {
    BufReader::new(File::open("src/day13/input.txt").unwrap()).lines()
        .map(Result::unwrap)
}