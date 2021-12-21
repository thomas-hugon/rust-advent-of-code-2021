use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::parser::{Packet, PacketType};

mod parser;

fn main() {
    let input = lines()
        .map(parse)
        .next().unwrap();

    println!("part1={}", input.version());
    println!("part2={}", input.eval());
}

fn parse(input: String) -> Packet {
    let vec = parser::str_to_hex(&input).ok().unwrap().1;
    parser::parse_packet(&vec).ok().unwrap().1
}

fn lines() -> impl Iterator<Item=String> {
    BufReader::new(File::open("src/day16/input.txt").unwrap()).lines()
        .map(Result::unwrap)
}

impl Packet {
    fn version(&self) -> u32 {
        match self {
            Packet::Literal { version, .. } => *version as u32,
            Packet::Operator { version, sub_packets, .. } =>
                *version as u32 + sub_packets.iter().map(Packet::version).sum::<u32>()
        }
    }
    fn eval(&self) -> u64 {
        match self {
            Packet::Literal { value, .. } => *value,
            Packet::Operator { operator, sub_packets, .. } => match operator {
                PacketType::Sum => sub_packets.iter().map(Packet::eval).sum(),
                PacketType::Product => sub_packets.iter().map(Packet::eval).product(),
                PacketType::Minimum => sub_packets.iter().map(Packet::eval).min().unwrap(),
                PacketType::Maximum => sub_packets.iter().map(Packet::eval).max().unwrap(),
                PacketType::GreaterThan if sub_packets[0].eval() > sub_packets[1].eval() => 1,
                PacketType::LessThan if sub_packets[0].eval() < sub_packets[1].eval() => 1,
                PacketType::EqualsTo if sub_packets[0].eval() == sub_packets[1].eval() => 1,
                _ => 0
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::parse;

    #[test]
    fn part1_8a004a801a8002f478() {
        assert_eq!(parse(String::from("8A004A801A8002F478")).version(), 16);
    }

    #[test]
    fn part1_620080001611562c8802118e34() {
        assert_eq!(parse(String::from("620080001611562C8802118E34")).version(), 12);
    }

    #[test]
    fn part1_c0015000016115a2e0802f182340() {
        assert_eq!(parse(String::from("C0015000016115A2E0802F182340")).version(), 23);
    }

    #[test]
    fn part1_a0016c880162017c3686b18a3d4780() {
        assert_eq!(parse(String::from("A0016C880162017C3686B18A3D4780")).version(), 31);
    }

    #[test]
    fn part2_c200b40a82() {
        assert_eq!(parse(String::from("C200B40A82")).eval(), 3);
    }

    #[test]
    fn part2_04005ac33890() {
        assert_eq!(parse(String::from("04005AC33890")).eval(), 54);
    }

    #[test]
    fn part2_880086c3e88112() {
        assert_eq!(parse(String::from("880086C3E88112")).eval(), 7);
    }

    #[test]
    fn part2_ce00c43d881120() {
        assert_eq!(parse(String::from("CE00C43D881120")).eval(), 9);
    }

    #[test]
    fn part2_d8005ac2a8f0() {
        assert_eq!(parse(String::from("D8005AC2A8F0")).eval(), 1);
    }

    #[test]
    fn part2_f600bc2d8f() {
        assert_eq!(parse(String::from("F600BC2D8F")).eval(), 0);
    }

    #[test]
    fn part2_9c005ac2f8f0() {
        assert_eq!(parse(String::from("9C005AC2F8F0")).eval(), 0);
    }

    #[test]
    fn part2_9c0141080250320f1802104a08() {
        assert_eq!(parse(String::from("9C0141080250320F1802104A08")).eval(), 1);
    }
}