use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let map = lines()
        .filter_map(|s| s.split_once("-").map(|(from, to)| (Cave::from(from), Cave::from(to))))
        .fold(HashMap::new(), |mut map, (from, to)| {
            map.entry(from.clone()).or_insert_with(|| Vec::new()).push(to.clone());
            map.entry(to).or_insert_with(|| Vec::new()).push(from);
            map
        });

    let paths_part1 = visit_next(&map, &Cave::Start, vec![&Cave::Start], Vec::new(), babar);
    let paths_part2 = visit_next(&map, &Cave::Start, vec![&Cave::Start], Vec::new(), celeste);

    println!("part1={:?}", paths_part1.len());
    println!("part2={:?}", paths_part2.len());
}

fn babar(already_visited_small_caves: &Vec<&Cave>, cave: &&Cave) -> bool {
    !already_visited_small_caves.contains(cave)
}

fn celeste(already_visited_small_caves: &Vec<&Cave>, cave: &&Cave) -> bool {
    let some_cave_already_visited_twice = already_visited_small_caves.into_iter()
        .fold(HashMap::new(), |mut counts, c| {
            *counts.entry(c).or_insert(0) += 1;
            counts
        }).values().any(|count| count > &1);
    cave != &&Cave::Start && (!some_cave_already_visited_twice || !already_visited_small_caves.contains(cave))
}


fn visit_next<'a>(map: &'a HashMap<Cave, Vec<Cave>>, current: &'a Cave, mut already_visited_small_caves: Vec<&'a Cave>, mut current_path: Vec<&'a Cave>, small_caves_validator: fn(&Vec<&Cave>, &&Cave) -> bool) -> Vec<Vec<&'a Cave>> {
    if let Cave::Small(_) = current {
        already_visited_small_caves.push(current);
    }
    current_path.push(current);
    if current == &Cave::End {
        return vec![current_path];
    }

    let nexts: Vec<_> = map.get(current).into_iter().flat_map(|vec| vec.into_iter())
        .filter(|cave| small_caves_validator(&already_visited_small_caves, cave))
        .collect();

    let mut paths = Vec::new();
    for next in nexts {
        let path = visit_next(map, next, already_visited_small_caves.clone(), current_path.clone(), small_caves_validator);
        if !path.is_empty() {
            paths.extend(path);
        }
    }

    paths
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
enum Cave {
    Start,
    End,
    Big(String),
    Small(String),
}

impl From<&str> for Cave {
    fn from(x: &str) -> Self {
        match x {
            "start" => Cave::Start,
            "end" => Cave::End,
            x if x.to_ascii_uppercase() == x => Cave::Big(x.to_string()),
            x if x.to_ascii_lowercase() == x => Cave::Small(x.to_string()),
            _ => panic!(),
        }
    }
}

fn lines() -> impl Iterator<Item=String> {
    BufReader::new(File::open("src/day12/input.txt").unwrap()).lines()
        .map(Result::unwrap)
}