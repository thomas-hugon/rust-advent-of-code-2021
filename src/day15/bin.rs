use std::convert::identity;
use std::fs::File;
use std::io::{BufRead, BufReader};

use pathfinding::prelude::dijkstra;

fn main() {
    let map: Vec<_> = lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
        .collect();

    let map_width_x_5 = map.iter().cloned()
        .map(|f| std::iter::successors(Some(f), |a| {
            Some(a.iter().map(|v| v % 9 + 1).collect::<Vec<_>>())
        }).take(5).flat_map(identity).collect::<Vec<_>>()).collect::<Vec<_>>();
    let map_width_x_5_height_x_5 = std::iter::successors(Some(map_width_x_5),|map|{
        Some(map.iter().map(|vec|vec.iter().map(|a|a % 9 + 1).collect::<Vec<_>>()).collect::<Vec<_>>())
    }).take(5).flat_map(identity).collect::<Vec<_>>();

    println!("part1={:?}", risk(map));
    println!("part2={:?}", risk(map_width_x_5_height_x_5));
}

fn risk(map: Vec<Vec<u32>>) -> u32 {
    let width = map.iter().max_by_key(|a| a.len()).unwrap().len() as i32;
    let height = map.len() as i32;
    let goal = (width as i32 - 1, height as i32 - 1);
    let result = dijkstra(&(0, 0),
                          |&(x, y)| [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
                              .into_iter()
                              .filter(|(x, y)| x >= &0 && y >= &0 && x < &width && y < &height)
                              .map(|p @ (x, y)| (p, map[y as usize][x as usize])),
                          |&p| p == goal);
    result.unwrap().1
}

fn lines() -> impl Iterator<Item=String> {
    BufReader::new(File::open("src/day15/input.txt").unwrap()).lines()
        .map(Result::unwrap)
}