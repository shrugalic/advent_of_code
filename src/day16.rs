use PayloadType::*;
use TypeId::*;

const INPUT: &str = include_str!("../input/day16.txt");

pub(crate) fn day16_part1() -> usize {
    Transmission::from(INPUT).version_sum
}

pub(crate) fn day16_part2() -> usize {
    Transmission::from(INPUT).packet.value
}

#[derive(Debug, PartialEq)]
struct Transmission {
    packet: Packet,
    version_sum: usize,
}
impl From<&str> for Transmission {
    fn from(input: &str) -> Self {
        let bits = hex_to_bits(input);
        let (packet, _, version_sum) = parse_packet(&bits);
        Transmission {
            packet,
            version_sum,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Packet {
    value: usize,
    packets: Vec<Packet>,
}
impl Packet {
    fn new(value: usize) -> Self {
        let packets = Vec::new();
        Packet { value, packets }
    }
}

type BitDigit = u8;

fn hex_to_bits(input: &str) -> Vec<BitDigit> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u8)
        .flat_map(|n| format!("{:04b}", n).chars().collect::<Vec<_>>())
        .map(|c| c.to_digit(10).unwrap() as BitDigit)
        .collect()
}

fn parse_packet(bits: &[BitDigit]) -> (Packet, usize, usize) {
    let (version, packet_type) = parse_header(bits);
    match packet_type {
        Literal => parse_literal_payload(bits, version),
        _ => parse_operator_payload(bits, version, packet_type),
    }
}

fn parse_header(bits: &[BitDigit]) -> (usize, TypeId) {
    let version = bits_to_number(&bits[0..3]);
    let type_id = TypeId::from(bits_to_number(&bits[3..6]));
    (version, type_id)
}

fn parse_literal_payload(bits: &[BitDigit], version: usize) -> (Packet, usize, usize) {
    let mut pos = 6;
    let mut value = vec![];
    for window in bits[pos..].windows(5).step_by(5) {
        value.extend_from_slice(&window[1..]);
        pos += 5;
        if window[0] == 0 {
            break;
        }
    }
    let value = bits_to_number(&value);
    (Packet::new(value), pos, version as usize)
}

fn parse_operator_payload(
    bits: &[BitDigit],
    version: usize,
    operator: TypeId,
) -> (Packet, usize, usize) {
    let (packets, pos, sum) = match PayloadType::from(&bits[6..]) {
        Bits(total_len) => parse_payload_bits(bits, total_len),
        Packets(count) => parse_payload_packets(bits, count),
    };
    let value = operator.applied_to(&packets);
    let packet = Packet { value, packets };
    (packet, pos, version as usize + sum)
}

fn parse_payload_bits(bits: &[BitDigit], total_len: usize) -> (Vec<Packet>, usize, usize) {
    let mut pos = 7 + 15;
    let mut version_sum = 0;
    let mut packets = vec![];
    let mut read_len = 0;
    while read_len < total_len {
        let (packet, len, sum) = parse_packet(&bits[pos..]);
        pos += len;
        read_len += len;
        version_sum += sum;
        packets.push(packet);
    }
    (packets, pos, version_sum)
}

fn parse_payload_packets(bits: &[BitDigit], packet_count: usize) -> (Vec<Packet>, usize, usize) {
    let mut pos = 7 + 11;
    let mut version_sum = 0;
    let mut packets = vec![];
    for _ in 0..packet_count {
        let (packet, len, sum) = parse_packet(&bits[pos..]);
        pos += len;
        version_sum += sum;
        packets.push(packet);
    }
    (packets, pos, version_sum)
}

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
impl From<usize> for TypeId {
    fn from(v: usize) -> Self {
        match v {
            0 => Sum,
            1 => Product,
            2 => Minimum,
            3 => Maximum,
            4 => Literal,
            5 => GreaterThan,
            6 => LessThan,
            7 => EqualTo,
            _ => unreachable!(),
        }
    }
}
impl TypeId {
    fn applied_to(&self, packets: &[Packet]) -> usize {
        let bool_to_num = |b| if b { 1 } else { 0 };
        match self {
            Sum => packets.iter().map(|p| p.value).sum(),
            Product => packets.iter().map(|p| p.value).product(),
            Minimum => packets.iter().map(|p| p.value).min().unwrap(),
            Maximum => packets.iter().map(|p| p.value).max().unwrap(),
            GreaterThan => bool_to_num(packets[0].value > packets[1].value),
            LessThan => bool_to_num(packets[0].value < packets[1].value),
            EqualTo => bool_to_num(packets[0].value == packets[1].value),
            Literal => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum PayloadType {
    Bits(usize),
    Packets(usize),
}
impl From<&[BitDigit]> for PayloadType {
    fn from(bits: &[BitDigit]) -> Self {
        match bits[0] {
            0 => Bits(bits_to_number(&bits[1..1 + 15])),
            _ => Packets(bits_to_number(&bits[1..1 + 11])),
        }
    }
}

fn bits_to_number(bits: &[BitDigit]) -> usize {
    let mut n = 0_usize;
    for bit in bits {
        n = (n << 1) + *bit as usize
    }
    n
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
        assert_eq!(Transmission::from("D2FE28").version_sum, 6);
        assert_eq!(Transmission::from("38006F45291200").version_sum, 9);
        assert_eq!(Transmission::from("EE00D40C823060").version_sum, 14);
    }

    #[test]
    fn parse_literal_packet() {
        let (packet, _, _) = parse_packet(&hex_to_bits("D2FE28"));
        assert_eq!(packet, Packet::new(2021));
    }

    #[test]
    fn parse_operator_packet_length_type_id_0() {
        let (packet, _, _) = parse_packet(&hex_to_bits("38006F45291200"));
        assert_eq!(
            packet,
            Packet {
                value: 1,
                packets: vec![Packet::new(10), Packet::new(20)],
            }
        );
    }

    #[test]
    fn parse_operator_packet_length_type_id_1() {
        let (packet, _, _) = parse_packet(&hex_to_bits("EE00D40C823060"));
        assert_eq!(
            packet,
            Packet {
                value: 3,
                packets: vec![Packet::new(1), Packet::new(2), Packet::new(3)],
            }
        );
    }

    #[test]
    fn parse_packet_example_1() {
        let (packet, _, _) = parse_packet(&hex_to_bits("8A004A801A8002F478"));
        assert_eq!(
            packet,
            Packet {
                value: 15,
                packets: vec![Packet {
                    value: 15,
                    packets: vec![Packet {
                        value: 15,
                        packets: vec![Packet::new(15)],
                    }],
                }],
            }
        );
    }

    #[test]
    fn parse_packet_example_2() {
        let (packet, _, _) = parse_packet(&hex_to_bits("620080001611562C8802118E34"));
        assert_eq!(
            packet,
            Packet {
                packets: vec![
                    Packet {
                        value: 10 + 11,
                        packets: vec![Packet::new(10), Packet::new(11)],
                    },
                    Packet {
                        value: 12 + 13,
                        packets: vec![Packet::new(12), Packet::new(13),],
                    }
                ],
                value: 21 + 25,
            }
        );
    }

    #[test]
    fn parse_packet_example_3() {
        let (packet, _, _) = parse_packet(&hex_to_bits("C0015000016115A2E0802F182340"));
        assert_eq!(
            packet,
            Packet {
                packets: vec![
                    Packet {
                        value: 10 + 11,
                        packets: vec![Packet::new(10), Packet::new(11)],
                    },
                    Packet {
                        value: 12 + 13,
                        packets: vec![Packet::new(12), Packet::new(13),],
                    }
                ],
                value: 21 + 25,
            }
        );
    }

    #[test]
    fn part1_examples() {
        assert_eq!(
            4 + 1 + 5 + 6,
            Transmission::from("8A004A801A8002F478").version_sum
        );
        assert_eq!(
            12,
            Transmission::from("620080001611562C8802118E34").version_sum
        );
        assert_eq!(
            23,
            Transmission::from("C0015000016115A2E0802F182340").version_sum
        );
        assert_eq!(
            31,
            Transmission::from("A0016C880162017C3686B18A3D4780").version_sum
        );
    }

    #[test]
    fn part1() {
        assert_eq!(908, day16_part1());
    }

    #[test]
    fn part2_examples_sum() {
        assert_eq!(1 + 2, Transmission::from("C200B40A82").packet.value);
    }
    #[test]
    fn part2_example_product() {
        assert_eq!(6 * 9, Transmission::from("04005AC33890").packet.value);
    }
    #[test]
    fn part2_example_min() {
        assert_eq!(7, Transmission::from("880086C3E88112").packet.value);
    }
    #[test]
    fn part2_example_max() {
        assert_eq!(9, Transmission::from("CE00C43D881120").packet.value);
    }
    #[test]
    fn part2_example_less_than() {
        assert_eq!(1, Transmission::from("D8005AC2A8F0").packet.value);
    }
    #[test]
    fn part2_example_greater_than() {
        assert_eq!(0, Transmission::from("F600BC2D8F").packet.value);
    }
    #[test]
    fn part2_example_equal() {
        assert_eq!(0, Transmission::from("9C005AC2F8F0").packet.value);
    }
    #[test]
    fn part2_example_combination() {
        assert_eq!(
            1,
            Transmission::from("9C0141080250320F1802104A08")
                .packet
                .value
        );
    }

    #[test]
    fn part2() {
        assert_eq!(10_626_195_124_371, day16_part2());
    }
}
