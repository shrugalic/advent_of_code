const INPUT: &str = include_str!("../input/day16.txt");

pub(crate) fn day16_part1() -> usize {
    let transmission = Transmission::from(INPUT);
    transmission.version_sum()
}

pub(crate) fn day16_part2() -> usize {
    let packet = Transmission::from(INPUT);
    packet.value()
}

type BitDigit = u8;
type HexDigit = u8;

#[derive(Debug, PartialEq, Clone)]
enum TypeId {
    Sum,
    Product,
    Minimum,
    Maximum,
    Literal,
    GreaterThan,
    LessThan,
    EqualTo,
}
impl From<u8> for TypeId {
    fn from(v: u8) -> Self {
        match v {
            0 => TypeId::Sum,
            1 => TypeId::Product,
            2 => TypeId::Minimum,
            3 => TypeId::Maximum,
            4 => TypeId::Literal,
            5 => TypeId::GreaterThan,
            6 => TypeId::LessThan,
            7 => TypeId::EqualTo,
            _ => unreachable!(),
        }
    }
}
impl TypeId {
    fn apply_to(&self, packets: &[Packet]) -> usize {
        let bool_to_num = |b| if b { 1 } else { 0 };
        match self {
            TypeId::Sum => packets.iter().map(|p| p.value).sum(),
            TypeId::Product => packets.iter().map(|p| p.value).product(),
            TypeId::Minimum => packets.iter().map(|p| p.value).min().unwrap(),
            TypeId::Maximum => packets.iter().map(|p| p.value).max().unwrap(),
            TypeId::GreaterThan => bool_to_num(packets[0].value > packets[1].value),
            TypeId::LessThan => bool_to_num(packets[0].value < packets[1].value),
            TypeId::EqualTo => bool_to_num(packets[0].value == packets[1].value),
            TypeId::Literal => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum LengthTypeId {
    Bits,
    Packets,
}
impl From<BitDigit> for LengthTypeId {
    fn from(b: BitDigit) -> Self {
        match b {
            0 => LengthTypeId::Bits,
            _ => LengthTypeId::Packets,
        }
    }
}
impl LengthTypeId {
    fn number_of_length_bits(&self) -> usize {
        match self {
            LengthTypeId::Bits => 15,
            LengthTypeId::Packets => 11,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Packet {
    version: u8,
    type_id: TypeId,
    value: usize,
    len: usize,
}
fn bits_to_number(bits: &[u8]) -> usize {
    let mut n = 0_usize;
    for bit in bits {
        n = (n << 1) + *bit as usize
    }
    n
}

#[derive(Debug, PartialEq)]
struct Transmission {
    packets: Vec<Packet>,
}
impl Transmission {
    fn version_sum(&self) -> usize {
        self.packets.iter().map(|p| p.version as usize).sum()
    }
    fn value(&self) -> usize {
        self.packets[0].value
    }
}
impl From<&str> for Transmission {
    fn from(input: &str) -> Self {
        let bits = hex_to_bits(input);
        let packets = parse_packets(&bits);
        Transmission { packets }
    }
}
fn parse_packets(bits: &[u8]) -> Vec<Packet> {
    let version = bits_to_number(&bits[0..3]) as u8;
    let type_id = TypeId::from(bits_to_number(&bits[3..6]) as u8);
    let mut pos = 6;
    match type_id {
        TypeId::Literal => {
            let mut value = vec![];
            while pos < bits.len() {
                let is_last_value = bits[pos] == 0;
                pos += 1;
                value.extend_from_slice(&bits[pos..pos + 4]);
                pos += 4;
                if is_last_value {
                    break;
                };
            }
            let value = bits_to_number(&value);
            vec![Packet {
                version,
                type_id,
                value,
                len: pos,
            }]
        }
        _ /* Operator */ => {
            let mut packets = vec![];

            let len_type = LengthTypeId::from(bits[pos]);
            pos += 1;
            let len_len = len_type.number_of_length_bits();
            let payload_len = bits_to_number(&bits[pos..pos + len_len]);
            pos += len_len;
            match len_type {
                LengthTypeId::Bits => {
                    let mut len = 0;
                    let mut all_sub_packets = vec![];
                    let mut first_only = vec![];
                    while len < payload_len {
                        let mut sub_packets = parse_packets(&bits[pos..]);
                        first_only.push(sub_packets[0].clone());
                        len += sub_packets[0].len;
                        pos += sub_packets[0].len;
                        all_sub_packets.append(&mut sub_packets);
                    }
                    let value = type_id.apply_to(&first_only);
                    packets.append(&mut all_sub_packets);
                    let packet = Packet {
                        version,
                        type_id,
                        value,
                        len: pos,
                    };
                    packets.insert(0, packet);
                }
                LengthTypeId::Packets => {
                    let mut all_sub_packets = vec![];
                    let mut first_only = vec![];
                    for _ in 0..payload_len {
                        let mut sub_packets = parse_packets(&bits[pos..]);
                        first_only.push(sub_packets[0].clone());
                        pos += sub_packets[0].len;
                        all_sub_packets.append(&mut sub_packets);
                    }
                    let value = type_id.apply_to(&first_only);
                    packets.append(&mut all_sub_packets);
                    let packet = Packet {
                        version,
                        type_id,
                        value,
                        len: pos,
                    };
                    packets.insert(0, packet);
                }
            }
            packets
        }
    }
}

fn hex_to_bits(input: &str) -> Vec<BitDigit> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(16).unwrap() as HexDigit)
        .flat_map(|n| format!("{:04b}", n).chars().collect::<Vec<_>>())
        .map(|c| c.to_digit(10).unwrap() as BitDigit)
        .collect()

    // half bytes
    /*
    for char in s.chars() {
        let value = char.to_digit(16).unwrap();
        println!("byte {} = {} = {:04b}", char, value, value);
    }
    */

    // full bytes
    /*
    let input: Vec<_> = s.trim().chars().collect();
    for byte in input.windows(2).step_by(2) {
        let value = u8::from_str_radix(&byte.iter().collect::<String>(), 16).unwrap();
        println!("byte {:?} = {} = {:08b}", byte, value, value);
    }
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_number() {
        assert_eq!(6, bits_to_number(&[1, 1, 0]));
        assert_eq!(4, bits_to_number(&[1, 0, 0]));
    }

    #[test]
    fn test_hex_to_bits() {
        assert_eq!(
            hex_to_bits("D2FE28"),
            vec![1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0],
        );
    }

    #[test]
    fn test_version_sum() {
        assert_eq!(Transmission::from("D2FE28").version_sum(), 6);
        assert_eq!(
            Transmission::from("38006F45291200").version_sum(),
            1 + 6 + 2,
        );
        assert_eq!(
            Transmission::from("EE00D40C823060").version_sum(),
            7 + 2 + 4 + 1,
        );
    }

    #[test]
    fn parse_literal_packet() {
        let bits = hex_to_bits("D2FE28");
        let packets = parse_packets(&bits[..]);
        assert_eq!(packets.len(), 1);
        assert_eq!(
            Packet {
                version: 6,
                type_id: TypeId::Literal,
                value: 2021,
                len: 21
            },
            packets[0]
        );
    }

    #[test]
    fn parse_operator_packet_length_type_id_0() {
        let bits = hex_to_bits("38006F45291200");
        let packets = parse_packets(&bits[..]);
        assert_eq!(packets.len(), 3);
        assert_eq!(
            packets[0],
            Packet {
                version: 1,
                type_id: TypeId::LessThan,
                value: 1,
                len: (3 + 3 + 1 + 15) + (11 + 16) // 49
            }
        );
        assert_eq!(
            packets[1],
            Packet {
                version: 6,
                type_id: TypeId::Literal,
                value: 10,
                len: 3 + 3 + 5
            }
        );
        assert_eq!(
            packets[2],
            Packet {
                version: 2,
                type_id: TypeId::Literal,
                value: 20,
                len: 3 + 3 + 5 + 5
            }
        );
    }

    #[test]
    fn parse_operator_packet_length_type_id_1() {
        let bits = hex_to_bits("EE00D40C823060");
        assert_eq!(56, bits.len());
        let packets = parse_packets(&bits[..]);
        assert_eq!(packets.len(), 4);
        assert_eq!(
            packets[0],
            Packet {
                version: 7,
                type_id: TypeId::Maximum,
                value: 3,
                len: (3 + 3 + 1 + 11) + (3 * 11)
            }
        );
        assert_eq!(
            packets[1],
            Packet {
                version: 2,
                type_id: TypeId::Literal,
                value: 1,
                len: 11
            }
        );
        assert_eq!(
            packets[2],
            Packet {
                version: 4,
                type_id: TypeId::Literal,
                value: 2,
                len: 11
            }
        );
        assert_eq!(
            packets[3],
            Packet {
                version: 1,
                type_id: TypeId::Literal,
                value: 3,
                len: 11
            }
        );
    }

    #[test]
    fn parse_packet_example_1() {
        let bits = hex_to_bits("8A004A801A8002F478");
        assert_eq!(72, bits.len());
        let packets = parse_packets(&bits[..]);
        assert_eq!(packets.len(), 4);
        assert_eq!(
            packets[0],
            Packet {
                version: 4,
                type_id: TypeId::Minimum,
                value: 15,
                len: (3 + 3 + 1 + 11) + 51
            }
        );
        assert_eq!(
            packets[1],
            Packet {
                version: 1,
                type_id: TypeId::Minimum,
                value: 15,
                len: (3 + 3 + 1 + 11) + 33 // 51
            }
        );
        assert_eq!(
            packets[2],
            Packet {
                version: 5,
                type_id: TypeId::Minimum,
                value: 15,
                len: (6 + 1 + 15) + 11 // 33
            }
        );
        assert_eq!(
            packets[3],
            Packet {
                version: 6,
                type_id: TypeId::Literal,
                value: 15,
                len: 3 + 3 + 5 // 11
            }
        );
    }

    #[test]
    fn parse_packet_example_2() {
        let bits = hex_to_bits("620080001611562C8802118E34");
        assert_eq!(104, bits.len());
        let packets = parse_packets(&bits[..]);
        assert_eq!(packets.len(), 7);
        assert_eq!(
            packets[0],
            Packet {
                version: 3,
                type_id: TypeId::Sum,
                value: 21 + 25, // 46
                len: (3 + 3 + 1 + 11) + 44 + 40
            }
        );
        assert_eq!(
            packets[1],
            Packet {
                version: 0,
                type_id: TypeId::Sum,
                value: 10 + 11,                  // 21
                len: (3 + 3 + 1 + 15) + 11 + 11  // 44
            }
        );
        assert_eq!(
            packets[2],
            Packet {
                version: 0,
                type_id: TypeId::Literal,
                value: 10,
                len: 3 + 3 + 5 // 11
            }
        );
        assert_eq!(
            packets[3],
            Packet {
                version: 5,
                type_id: TypeId::Literal,
                value: 11,
                len: 3 + 3 + 5 // 11
            }
        );
        assert_eq!(
            packets[4],
            Packet {
                version: 1,
                type_id: TypeId::Sum,
                value: 12 + 13,                  // 25
                len: (3 + 3 + 1 + 11) + 11 + 11  // 40
            }
        );
        assert_eq!(
            packets[5],
            Packet {
                version: 0,
                type_id: TypeId::Literal,
                value: 12,
                len: 3 + 3 + 5 // 11
            }
        );
        assert_eq!(
            packets[6],
            Packet {
                version: 3,
                type_id: TypeId::Literal,
                value: 13,
                len: 3 + 3 + 5 // 11
            }
        );
    }

    #[test]
    fn parse_packet_example_3() {
        let bits = hex_to_bits("C0015000016115A2E0802F182340");
        assert_eq!(112, bits.len());
        let packets = parse_packets(&bits[..]);
        assert_eq!(packets.len(), 7);
        assert_eq!(
            packets[0],
            Packet {
                version: 6,
                type_id: TypeId::Sum,
                value: 21 + 25, // 46
                len: (3 + 3 + 1 + 15) + 44 + 40
            }
        );
        assert_eq!(
            packets[1],
            Packet {
                version: 0,
                type_id: TypeId::Sum,
                value: 10 + 11,                  // 21
                len: (3 + 3 + 1 + 15) + 11 + 11  // 44
            }
        );
        assert_eq!(
            packets[2],
            Packet {
                version: 0,
                type_id: TypeId::Literal,
                value: 10,
                len: 3 + 3 + 5 // 11
            }
        );
        assert_eq!(
            packets[3],
            Packet {
                version: 6,
                type_id: TypeId::Literal,
                value: 11,
                len: 3 + 3 + 5 // 11
            }
        );
        assert_eq!(
            packets[4],
            Packet {
                version: 4,
                type_id: TypeId::Sum,
                value: 12 + 13,                  // 25
                len: (3 + 3 + 1 + 11) + 11 + 11  // 40
            }
        );
        assert_eq!(
            packets[5],
            Packet {
                version: 7,
                type_id: TypeId::Literal,
                value: 12,
                len: 3 + 3 + 5 // 11
            }
        );
        assert_eq!(
            packets[6],
            Packet {
                version: 0,
                type_id: TypeId::Literal,
                value: 13,
                len: 3 + 3 + 5 // 11
            }
        );
    }

    #[test]
    fn part1_example_1() {
        assert_eq!(
            4 + 1 + 5 + 6,
            Transmission::from("8A004A801A8002F478").version_sum()
        );
    }
    #[test]
    fn part1_example_2() {
        assert_eq!(
            12,
            Transmission::from("620080001611562C8802118E34").version_sum()
        );
    }
    #[test]
    fn part1_example_3() {
        assert_eq!(
            23,
            Transmission::from("C0015000016115A2E0802F182340").version_sum()
        );
    }
    #[test]
    fn part1_example_4() {
        assert_eq!(
            31,
            Transmission::from("A0016C880162017C3686B18A3D4780").version_sum()
        );
    }

    #[test]
    fn part1() {
        assert_eq!(908, day16_part1());
    }

    #[test]
    fn part2_example_sum() {
        assert_eq!(1 + 2, Transmission::from("C200B40A82").value());
    }
    #[test]
    fn part2_example_product() {
        assert_eq!(6 * 9, Transmission::from("04005AC33890").value());
    }
    #[test]
    fn part2_example_min() {
        assert_eq!(7, Transmission::from("880086C3E88112").value());
    }
    #[test]
    fn part2_example_max() {
        assert_eq!(9, Transmission::from("CE00C43D881120").value());
    }
    #[test]
    fn part2_example_less_than() {
        assert_eq!(1, Transmission::from("D8005AC2A8F0").value());
    }
    #[test]
    fn part2_example_greater_than() {
        assert_eq!(0, Transmission::from("F600BC2D8F").value());
    }
    #[test]
    fn part2_example_equal() {
        assert_eq!(0, Transmission::from("9C005AC2F8F0").value());
    }
    #[test]
    fn part2_example_combination() {
        assert_eq!(1, Transmission::from("9C0141080250320F1802104A08").value());
    }

    #[test]
    fn part2() {
        assert_eq!(10_626_195_124_371, day16_part2());
    }
}
