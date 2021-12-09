use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct FishGroup(pub u8, pub u64);

impl FishGroup {
    fn tick(&mut self) -> Option<Self> {
        if self.0 == 0 {
            self.0 = 6;
            self.1;
            Some(FishGroup(8, self.1))
        } else {
            self.0 -= 1;
            None
        }
    }

    fn merge(&mut self, other: Self) {
        if self.0 != other.0 {
            panic!("different timers");
        }
        self.1 += other.1;
    }
}


fn main() {
    println!("part1={}", fishes(80));
    println!("part2={}", fishes(256));
}

fn fishes(nb_days: i32) -> u64 {
    let mut fishes = lines()
        .flat_map(|s| s.split(",").map(|s| s.parse::<u8>().unwrap()).collect::<Vec<_>>())
        .map(|timer| FishGroup(timer, 1))
        .collect::<Vec<_>>();

    for _ in 0..nb_days {
        fishes.sort_unstable_by_key(|f| f.0);
        fishes = fishes.into_iter()
            .fold(Vec::new(), |mut vec: Vec<FishGroup>, fg| {
                match vec.last_mut() {
                    Some(fg2) if fg2.0 == fg.0 => fg2.merge(fg),
                    _ => vec.push(fg),
                }
                vec
            });
        let newborns: Vec<_> = fishes.iter_mut()
            .filter_map(|f| f.tick())
            .collect();
        fishes.extend(newborns);
    }

    fishes.iter().map(|fg| fg.1).sum::<u64>()
}

fn lines() -> impl Iterator<Item=String> {
    BufReader::new(File::open("src/day6/input.txt").unwrap()).lines()
        .map(Result::unwrap)
}