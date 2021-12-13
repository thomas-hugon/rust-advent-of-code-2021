use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let map = lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let minima = find_minima(&map);
    let risk_level = risk_level(&minima);

    let mut basins = minima.into_iter()
        .map(|Minimum { point, .. }| {
            let mut set = HashSet::new();
            flow(&map, point, &mut set);
            set
        })
        .collect::<Vec<_>>();
    basins.sort_unstable_by(|a, b| b.len().cmp(&a.len()));

    let product_of_sizes_of_3_largest = &basins[0..=2].iter().map(|b| b.len()).product::<usize>();

    println!("part1={:?}", risk_level);
    println!("part2={:?}", product_of_sizes_of_3_largest);
}

fn flow(map: &Vec<Vec<u8>>, destination: Point, vec: &mut HashSet<Point>) {
    let v_pos = destination.v_pos;
    let h_pos = destination.h_pos;
    if vec.contains(&destination) || !can_flow(map, v_pos, h_pos) {
        return;
    }
    vec.insert(destination);
    for (x, y) in [(1, 0), (0, 1)] {
        flow(map, Point { v_pos: v_pos + x, h_pos: h_pos + y }, vec);
        flow(map, Point { v_pos: v_pos.saturating_sub(x), h_pos: h_pos.saturating_sub(y) }, vec);
    }
}

fn can_flow(map: &Vec<Vec<u8>>, v_pos: usize, h_pos: usize) -> bool {
    map.get(v_pos).and_then(|vec| vec.get(h_pos)).filter(|value| value != &&9).is_some()
}

fn risk_level(minima: &Vec<Minimum>) -> u32 {
    minima.into_iter()
        .map(|Minimum { value, .. }| *value as u32 + 1)
        .sum()
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    h_pos: usize,
    v_pos: usize,
}

#[derive(Debug)]
struct Minimum {
    point: Point,
    value: u8,
}

fn find_minima(map: &Vec<Vec<u8>>) -> Vec<Minimum> {
    map.into_iter().enumerate()
        .flat_map(|(v_pos, line)| line.into_iter().enumerate().map(move |(h_pos, value)| (v_pos, h_pos, value)))
        .filter(|(v_pos, h_pos, value)|
            [(1, 0), (0, 1)].iter().all(|(x, y)| !is_lesser_than(map, v_pos + x, h_pos + y, **value)
                && !is_lesser_than(map, v_pos.wrapping_sub(*x), h_pos.wrapping_sub(*y), **value)
            ))
        .map(|(v_pos, h_pos, value)| Minimum { point: Point { h_pos, v_pos }, value: *value })
        .collect()
}

fn is_lesser_than(map: &Vec<Vec<u8>>, v_pos: usize, h_pos: usize, value: u8) -> bool {
    get(map, v_pos, h_pos).filter(|&&x| x <= value).is_some()
}

fn get(map: &Vec<Vec<u8>>, v_pos: usize, h_pos: usize) -> Option<&u8> {
    map.get(v_pos).and_then(|vec| vec.get(h_pos))
}

fn lines() -> impl Iterator<Item=String> {
    BufReader::new(File::open("src/day9/input.txt").unwrap()).lines()
        .map(Result::unwrap)
}