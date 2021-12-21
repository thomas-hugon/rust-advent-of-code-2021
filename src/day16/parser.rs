use nom::IResult;

mod str {
    use nom::bytes::complete::take_while_m_n;
    use nom::combinator::map_res;
    use nom::IResult;
    use nom::multi::many0;

    fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
        u8::from_str_radix(input, 16)
    }

    fn is_hex_digit(c: char) -> bool {
        c.is_digit(16)
    }

    fn hex_u8(input: &str) -> IResult<&str, u8> {
        map_res(
            take_while_m_n(2, 2, is_hex_digit),
            from_hex,
        )(input)
    }

    pub(crate) fn hex(input: &str) -> IResult<&str, Vec<u8>> {
        many0(hex_u8)(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parse_hex_str_to_vec_u8() {
            assert_eq!(hex("2F14DF"), Ok(("", vec![0x2F, 0x14, 0xDF])));
            assert_eq!(hex("D2FE28"), Ok(("", vec![0xD2, 0xFE, 0x28])));
        }
    }
}

mod bits {
    use nom::{IResult, Parser};
    use nom::bits::complete::tag;
    use nom::bits::complete::take;
    use nom::combinator::{flat_map, map};
    use nom::error::ParseError;
    use nom::multi::{length_count, many0, many_m_n};
    use nom::sequence::preceded;

    use crate::parser::{Packet, PacketType};

    pub fn combine_vec_parser<I, O, E: ParseError<I>, F, G>(mut first: F, mut second: G)
                                                            -> impl FnMut(I) -> IResult<I, Vec<O>, E>
        where
            F: Parser<I, Vec<O>, E>,
            G: Parser<I, O, E>,
    {
        move |input: I| {
            let (input, mut o1) = first.parse(input)?;
            second.parse(input).map(|(i, o2)| {
                o1.push(o2);
                (i, o1)
            })
        }
    }

    pub(crate) fn take_bits_vec_u8(bits_len: usize) -> impl FnMut((&[u8], usize)) -> IResult<(&[u8], usize), Vec<u8>> {
        move |i| {
            let nb_bytes = bits_len / 8;
            let nb_remaining_bits = bits_len % 8;
            if nb_remaining_bits == 0 {
                many_m_n(nb_bytes, nb_bytes, take::<_, u8, _, _>(8usize))(i)
            } else {
                combine_vec_parser(
                    many_m_n(nb_bytes, nb_bytes, take::<_, u8, _, _>(8usize)),
                    take::<_, u8, _, _>(nb_remaining_bits).map(|x| x << (8 - nb_remaining_bits)),
                )(i)
            }
        }
    }

    fn packet_version(i: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
        take(3usize)(i)
    }

    fn packet_type(i: (&[u8], usize)) -> IResult<(&[u8], usize), PacketType> {
        map(take(3usize),
            |value: u8| match value {
                0 => PacketType::Sum,
                1 => PacketType::Product,
                2 => PacketType::Minimum,
                3 => PacketType::Maximum,
                5 => PacketType::GreaterThan,
                6 => PacketType::LessThan,
                7 => PacketType::EqualsTo,
                _ => PacketType::Literal
            },
        )(i)
    }

    pub(crate) fn packet(i: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
        let (i, version) = packet_version(i)?;
        let (i, ptype) = packet_type(i)?;
        match ptype {
            PacketType::Literal => packet_literal(version)(i),
            operator => packet_operator(version, operator)(i)
        }
    }

    pub(crate) fn packet_literal(version: u8) -> impl FnMut((&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
        move |i| {
            map(packet_literal_value,
                |value| Packet::Literal { version, value },
            )(i)
        }
    }

    pub(crate) fn packet_literal_value(i: (&[u8], usize)) -> IResult<(&[u8], usize), u64> {
        let unfinished_4bits = preceded(tag(1, 1usize), take::<_, u8, _, _>(4usize));
        let finished_4bits = preceded(tag(0, 1usize), take::<_, u8, _, _>(4usize));
        map(combine_vec_parser(many0(unfinished_4bits), finished_4bits),
            |vec| vec.iter().fold(0u64, |acc, val| (acc << 4u64) | (*val as u64)),
        )(i)
    }

    fn packet_operator(version: u8, operator: PacketType) -> impl FnMut((&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
        move |i| {
            map(sub_packets,
                |sub_packets| Packet::Operator { version, operator, sub_packets },
            )(i)
        }
    }

    fn sub_packets(i: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<Packet>> {
        let (i, type_subpackets_list) = take::<_, u8, _, _>(1usize)(i)?;
        match type_subpackets_list {
            0 => total_length_sub_packets(i),
            _ => numbered_subpackets(i),
        }
    }

    fn numbered_subpackets(i: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<Packet>> {
        length_count(take::<_, usize, _, _>(11usize), packet)(i)
    }

    fn total_length_sub_packets(i: (&[u8], usize)) -> IResult<(&[u8], usize), Vec<Packet>> {
        let (j, vec) = flat_map(
            take::<_, usize, _, _>(15usize),
            take_bits_vec_u8,
        )(i)?;

        let result = many0(packet)((&vec, 0usize))
            .map(|(_, sub_packets)| (j, sub_packets))
            .map_err(|e| e.map_input(|_| j));
        result
    }

    #[cfg(test)]
    mod tests {
        use nom::error::{Error, ErrorKind};

        use super::*;

        #[test]
        fn packet_version_ok() {
            assert_eq!(
                packet_version((&vec![0b10111111, 0b10001010][..], 0usize)),
                Ok(((&[0b10111111, 0b10001010][..], 3usize), 5))
            );
            assert_eq!(
                packet_version((&vec![0b00011111, 0b10001010][..], 0usize)),
                Ok(((&[0b00011111, 0b10001010][..], 3usize), 0))
            );
        }

        #[test]
        fn packet_type_ok() {
            assert_eq!(
                packet_type((&vec![0b10111111, 0b10001010][..], 0usize)),
                Ok(((&[0b10111111, 0b10001010][..], 3usize), PacketType::GreaterThan))
            );
            assert_eq!(
                packet_type((&vec![0b00011111, 0b10001010][..], 0usize)),
                Ok(((&[0b00011111, 0b10001010][..], 3usize), PacketType::Sum))
            );
            assert_eq!(
                packet_type((&vec![0b10011111, 0b10001010][..], 0usize)),
                Ok(((&[0b10011111, 0b10001010][..], 3usize), PacketType::Literal))
            );
        }

        #[test]
        fn packet_literal_ok_2021() {
            assert_eq!(
                packet_literal(6)((&vec![0b10111111, 0b10001010][..], 0usize)),
                Ok(((&[0b10001010][..], 7usize), Packet::Literal { version: 6, value: 2021 }))
            );
        }

        #[test]
        fn packet_literal_ok_32351() {
            assert_eq!(
                packet_literal(2)((&vec![0b10111111, 0b10101010, 0b11110000][..], 0usize)),
                Ok(((&[0b11110000][..], 4usize), Packet::Literal { version: 2, value: 32351 }))
            );
        }

        #[test]
        fn packet_literal_ko_no_literal_end_part_before_eof() {
            assert_eq!(
                packet_literal(32)((&vec![0b10111111, 0b10101011, 0b11111000, 0b01000010, 0b00010000][..], 0usize)),
                Err(nom::Err::Error(Error { input: (&[][..], 0), code: ErrorKind::Eof }))
            );
        }

        #[test]
        fn total_length_sub_packets_ok() {
            // 38006F45291200
            assert_eq!(
                total_length_sub_packets((&vec![0b00000000, 0b00110111, 0b10100010, 0b10010100, 0b10001001, 0b00111111][..], 0usize)),
                Ok(((&[0b00111111][..], 2usize), vec![Packet::Literal { version: 6, value: 10 }, Packet::Literal { version: 2, value: 20 }]))
            );
        }

        #[test]
        fn numbered_subpackets_ok() {
            // 38006F45291200
            assert_eq!(
                numbered_subpackets((&vec![0b00000000, 0b01101010, 0b00000110, 0b01000001, 0b00011000, 0b00110111][..], 0usize)),
                Ok(((&[0b00110111][..], 4usize), vec![
                    Packet::Literal { version: 2, value: 1 },
                    Packet::Literal { version: 4, value: 2 },
                    Packet::Literal { version: 1, value: 3 },
                ]))
            );
        }

        #[test]
        fn packet_operator_1() {
            assert_eq!(
                packet_operator(1, PacketType::Sum)((&vec![0b00000000, 0b00011011, 0b11010001, 0b01001010, 0b01000100, 0b10000000][..], 0usize)),
                Ok(((&[0b10000000][..], 3usize), Packet::Operator {
                    version: 1,
                    operator: PacketType::Sum,
                    sub_packets:
                    vec![Packet::Literal { version: 6, value: 10 }, Packet::Literal { version: 2, value: 20 }],
                }))
            );
        }

        #[test]
        fn packet_operator_2() {
            // 38006F45291200
            assert_eq!(
                packet_operator(7, PacketType::Sum)((&vec![0b10000000, 0b00110101, 0b00000011, 0b00100000, 0b10001100, 0b00011000][..], 0usize)),
                Ok(((&[0b00011000][..], 5usize), Packet::Operator {
                    version: 7,
                    operator: PacketType::Sum,
                    sub_packets:
                    vec![
                        Packet::Literal { version: 2, value: 1 },
                        Packet::Literal { version: 4, value: 2 },
                        Packet::Literal { version: 1, value: 3 },
                    ],
                }))
            );
        }

        #[test]
        fn packet_2() {
            // 38006F45291200
            assert_eq!(
                packet((&vec![0xEE, 0x00, 0xD4, 0x0C, 0x82, 0x30, 0x60][..], 0usize)),
                Ok(((&[0x60][..], 3usize), Packet::Operator {
                    version: 7,
                    operator: PacketType::Maximum,
                    sub_packets:
                    vec![
                        Packet::Literal { version: 2, value: 1 },
                        Packet::Literal { version: 4, value: 2 },
                        Packet::Literal { version: 1, value: 3 },
                    ],
                }))
            );
        }

        #[test]
        fn packet_1() {
            assert_eq!(
                packet((&vec![0x38, 0x00, 0x6F, 0x45, 0x29, 0x12, 0x00][..], 0usize)),
                Ok(((&[0x00][..], 1usize), Packet::Operator {
                    version: 1,
                    operator: PacketType::LessThan,
                    sub_packets:
                    vec![Packet::Literal { version: 6, value: 10 }, Packet::Literal { version: 2, value: 20 }],
                }))
            );
        }
    }
}

pub(crate) fn str_to_hex(i: &str) -> IResult<&str, Vec<u8>> {
    str::hex(i)
}

pub(crate) fn parse_packet(i: &[u8]) -> IResult<&[u8], Packet> {
    nom::bits(bits::packet)(i)
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub(crate) enum Packet {
    Literal { version: u8, value: u64 },
    Operator { version: u8, operator:PacketType, sub_packets: Vec<Packet> },
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub(crate) enum PacketType {
    Literal,
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualsTo,
}
