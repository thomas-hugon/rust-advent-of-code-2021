use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn main() {
    let mut lines = lines();
    let mut tuple_counts = lines.next().unwrap().chars().collect::<Vec<_>>().iter().cloned().tuple_windows::<(char, char)>().counts();
    let map = lines.filter_map(|line| line.split_once(" -> ").map(|(a, b)| (a.to_string(), b.to_string())))
        .fold(HashMap::new(), |mut map, (a, b)| {
            let mut chars = a.chars();
            map.insert((chars.next().unwrap(), chars.next().unwrap()), b.chars().next().unwrap());
            map
        });


    for i in 0..40 {
        tuple_counts = tuple_counts.iter().flat_map(|((c1, c2), cnt)| {
            let c = map.get(&(*c1, *c2)).unwrap();
            [((c1, c), cnt), ((c, c2), cnt)]
        }).fold(HashMap::new(), |mut acc, ((c1, c2), cnt)| {
            *acc.entry((*c1, *c2)).or_insert(0) += *cnt;
            acc
        });

        let real_char_count = tuple_counts.iter().map(|((_, b), cnt)| (b, cnt))
            .fold(HashMap::new(), |mut acc, (c, cnt)| {
                *acc.entry(*c).or_insert(0) += cnt;
                acc
            });
        let ((_, min2), (_, max2)) = real_char_count.iter()
            .minmax_by_key(|(_, count)| **count)
            .into_option().unwrap();
        if i == 9 {
            println!("part1={:?}", (max2 - min2));
        } else if i == 39 {
            println!("part2={:?}", (max2 - min2));
        }
    }
}

fn lines() -> impl Iterator<Item=String> {
    BufReader::new(File::open("src/day14/input.txt").unwrap()).lines()
        .map(Result::unwrap)
}