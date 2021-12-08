use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("part1={}", part1());
    println!("part2={}", part2());
}

fn part1() -> u32 {
    let (count, mut sum) = lines()
        .map(|a| a.chars().collect::<Vec<_>>())
        .fold((0, vec![0; 12]), |(count, mut vec_agg), b| {
            for (a, b) in vec_agg.iter_mut().zip(b.iter()) {
                *a += b.to_digit(2).unwrap();
            };
            (count + 1, vec_agg)
        });
    for a in sum.iter_mut() {
        *a = (2 * *a) / count;
    }
    let c = sum.iter()
        .map(|x| char::from_digit(*x, 2).unwrap())
        .collect::<String>();

    let gamma = u32::from_str_radix(&c, 2).unwrap();
    let epsilon = 0xFFF ^ gamma;
    gamma * epsilon
}

fn part2() -> u32 {
    let mut most_common_vec = lines()
        .map(|a| a.chars().map(|c| c.to_digit(2).unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut least_common_vec = most_common_vec.clone();

    for i in 0..12 {
        let most_common = (2u32 * most_common_vec.iter().map(|a| a[i]).sum::<u32>()) / most_common_vec.len() as u32;
        most_common_vec.retain(|a| a[i] == most_common);
        if most_common_vec.len() == 1 {
            break;
        }
    }
    for i in 0..12 {
        let least_common = 1 ^ ((2u32 * least_common_vec.iter().map(|a| a[i]).sum::<u32>()) / least_common_vec.len() as u32);
        least_common_vec.retain(|a| a[i] == least_common);
        if least_common_vec.len() == 1 {
            break;
        }
    }

    let oxygen_generator_rating = u32::from_str_radix(&*most_common_vec[0].iter()
        .map(|x| char::from_digit(*x, 2).unwrap())
        .collect::<String>(), 2).unwrap();
    let co2_scrubber_rating = u32::from_str_radix(&*least_common_vec[0].iter()
        .map(|x| char::from_digit(*x, 2).unwrap())
        .collect::<String>(), 2).unwrap();

    oxygen_generator_rating * co2_scrubber_rating
}

fn lines() -> impl Iterator<Item=String> {
    BufReader::new(File::open("src/day3/input.txt").unwrap()).lines()
        .map(Result::unwrap)
}