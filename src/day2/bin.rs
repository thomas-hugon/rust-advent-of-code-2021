use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let part1: SubmarineWithoutAim = process_input();
    let part2: SubmarineWithAim = process_input();
    println!("part1={}\npart2={}", part1.result(), part2.result());
}

trait Submarine {
    fn forward(&mut self, forward: i32);
    fn up(&mut self, aim: i32);
    fn down(&mut self, aim: i32);
    fn result(&self) -> i32;
}

#[derive(Default, Debug)]
struct SubmarineWithoutAim {
    h_pos: i32,
    depth: i32,
}

impl Submarine for SubmarineWithoutAim {
    fn forward(&mut self, forward: i32) {
        self.h_pos += forward;
    }
    fn up(&mut self, aim: i32) {
        self.depth -= aim;
    }
    fn down(&mut self, aim: i32) {
        self.depth += aim;
    }
    fn result(&self) -> i32 {
        self.depth * self.h_pos
    }
}

#[derive(Default, Debug)]
struct SubmarineWithAim {
    h_pos: i32,
    depth: i32,
    aim: i32,
}

impl Submarine for SubmarineWithAim {
    fn forward(&mut self, forward: i32) {
        self.h_pos += forward;
        self.depth += self.aim * forward;
    }
    fn up(&mut self, aim: i32) {
        self.aim -= aim;
    }
    fn down(&mut self, aim: i32) {
        self.aim += aim;
    }
    fn result(&self) -> i32 {
        self.depth * self.h_pos
    }
}

fn process_input<T: Submarine + Default>() -> T {
    BufReader::new(File::open("src/day2/input.txt").unwrap()).lines()
        .fold(T::default(), |mut submarine, val| {
            match &val.unwrap().split_whitespace().collect::<Vec<&str>>()[..] {
                ["forward", forward] => submarine.forward(forward.parse().unwrap()),
                ["down", down] => submarine.down(down.parse().unwrap()),
                ["up", up] => submarine.up(up.parse().unwrap()),
                _ => panic!()
            };
            submarine
        })
}