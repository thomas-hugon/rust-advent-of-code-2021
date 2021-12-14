use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut map = lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut nb_flashed = 0;
    for i in 1..9999999 {
        for val in map.iter_mut().flat_map(|l| l.iter_mut()) {
            *val += 1;
        }
        let mut flash: HashSet<(usize, usize)> = HashSet::new();
        loop {
            let new_flash = map.iter_mut().enumerate()
                .flat_map(|(y, l)| l.iter_mut().enumerate()
                    .filter(|(_, val)| **val > 9)
                    .map(move |(x, _)| (x, y))
                )
                .filter(|(x, y)| !flash.contains(&(*x, *y)))
                .collect::<HashSet<_>>();

            if new_flash.is_empty() { break; }
            nb_flashed += new_flash.len();

            inc_neighbours(&mut map, &new_flash);

            flash.extend(new_flash.into_iter());
        }
        for val in map.iter_mut()
            .flat_map(|l| l.iter_mut())
            .filter(|val| **val > 9) {
            *val = 0;
        }

        if i == 100{
            println!("part1={}", nb_flashed);
        }
        if map.iter_mut().flat_map(|l| l.iter_mut()).all(|val| val == &0){
            println!("part2={}", i);
            break;
        }
    }
}

fn inc_neighbours(map: &mut Vec<Vec<u32>>, flashes: &HashSet<(usize, usize)>) {
    const NEIGHBOURS: [(isize, isize); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
    for (x, y) in flashes.iter().flat_map(|(x, y)|
        NEIGHBOURS.iter().map(|(nb_x, nb_y)| (*x as isize + nb_x, *y as isize + nb_y))
            .filter(|(x, y)| *x >= 0 && *y >= 0)
    ) {
        if let Some(val) = map.get_mut(y as usize).and_then(|v| v.get_mut(x as usize)) {
            *val += 1;
        }
    }
}

fn lines() -> impl Iterator<Item=String> {
    BufReader::new(File::open("src/day11/input.txt").unwrap()).lines()
        .map(Result::unwrap)
}