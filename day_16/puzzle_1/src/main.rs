use std::{error::Error, fs, slice::Iter};

const INPUT_PATH: &str = "input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let raw_input = fs::read_to_string(INPUT_PATH)?;
    let input: Vec<&str> = raw_input.lines().map(|line| line.trim()).collect();

    let output = compute(&input)?;

    println!("Puzzle output: {}", output);
    Ok(())
}

fn compute(input: &[&str]) -> Result<u32, Box<dyn Error>> {
    // parse to binary
    let mut binary: Vec<u8> = Vec::new();
    for val in input[0].chars() {
        match val {
            '0' => binary.extend_from_slice(&[0, 0, 0, 0]),
            '1' => binary.extend_from_slice(&[0, 0, 0, 1]),
            '2' => binary.extend_from_slice(&[0, 0, 1, 0]),
            '3' => binary.extend_from_slice(&[0, 0, 1, 1]),
            '4' => binary.extend_from_slice(&[0, 1, 0, 0]),
            '5' => binary.extend_from_slice(&[0, 1, 0, 1]),
            '6' => binary.extend_from_slice(&[0, 1, 1, 0]),
            '7' => binary.extend_from_slice(&[0, 1, 1, 1]),
            '8' => binary.extend_from_slice(&[1, 0, 0, 0]),
            '9' => binary.extend_from_slice(&[1, 0, 0, 1]),
            'A' => binary.extend_from_slice(&[1, 0, 1, 0]),
            'B' => binary.extend_from_slice(&[1, 0, 1, 1]),
            'C' => binary.extend_from_slice(&[1, 1, 0, 0]),
            'D' => binary.extend_from_slice(&[1, 1, 0, 1]),
            'E' => binary.extend_from_slice(&[1, 1, 1, 0]),
            'F' => binary.extend_from_slice(&[1, 1, 1, 1]),
            _ => unreachable!(),
        }
    }

    let packets = parse_packets(&mut binary.iter(), None);

    let mut version_sum = 0;
    for packet in packets {
        version_sum += packet.get_version_sum();
    }
    // do some more computation on packets
    Ok(version_sum)
}

#[derive(Debug)]
enum Packet {
    LiteralValue(LiteralValuePacket),
    Operator(OperatorPacket),
}

impl Packet {
    fn get_length(&self) -> u32 {
        match self {
            Packet::LiteralValue(packet) => packet.length,
            Packet::Operator(packet) => packet.length,
        }
    }

    fn get_version(&self) -> u32 {
        match self {
            Packet::LiteralValue(packet) => packet.header.version,
            Packet::Operator(packet) => packet.header.version,
        }
    }

    fn get_version_sum(&self) -> u32 {
        let mut value = 0;
        match self {
            Packet::LiteralValue(_) => value += self.get_version(),
            Packet::Operator(packet) => {
                value += self.get_version();
                for subpacket in &packet.subpackets {
                    value += subpacket.get_version_sum();
                }
            }
        }
        value
    }
}

#[derive(Debug)]
struct PacketHeader {
    version: u32,
    type_id: u32,
    length: u32,
}

impl PacketHeader {
    fn from_iter(iter: &mut Iter<u8>) -> Self {
        let length = 6;

        let mut version = 0;

        parse_value(3, &mut version, iter);

        let mut type_id = 0;
        parse_value(3, &mut type_id, iter);

        Self {
            version,
            type_id,
            length,
        }
    }
}

#[derive(Debug)]
struct LiteralValuePacket {
    header: PacketHeader,
    value: u32,
    length: u32,
}

impl LiteralValuePacket {
    fn from_iter(
        iter: &mut Iter<u8>,
        header: PacketHeader,
        subpacket_limit: Option<SubpacketLimit>,
    ) -> Self {
        // need to get 5 bits at a time
        // check first bit if we should continue
        // parse next 4 bits into a number (bit shift them)
        let mut value = 0;
        let mut is_more = true;

        let mut bits_taken = header.length;

        while is_more {
            bits_taken += 5;
            is_more = *iter
                .next()
                .expect("Found None while parsing LiteralValuePacket")
                == 1;
            parse_value(4, &mut value, iter);
        }

        // if we're not a subpacket then we should cull our bits
        if subpacket_limit.is_none() {
            // we may have only taken 11 bits, so we should move the other 5 0 bits off the end
            let remainder_bits = 16 - (bits_taken % 16);
            for _ in 0..remainder_bits {
                iter.next();
            }
            bits_taken += remainder_bits;
        }

        Self {
            header,
            value,
            length: bits_taken,
        }
    }
}

#[derive(Debug)]
struct OperatorPacket {
    header: PacketHeader,
    length: u32,
    length_type_id: u8,
    subpackets: Vec<Packet>,
}

impl OperatorPacket {
    fn from_iter(
        iter: &mut Iter<u8>,
        header: PacketHeader,
        subpacket_limit: Option<SubpacketLimit>,
    ) -> Self {
        let length_type_id = *iter
            .next()
            .expect("Found None while parsing OperatorPacket length_type_id");

        let mut bits_taken = header.length + 1;
        let mut subpackets = Vec::new();

        let mut value = 0;
        if length_type_id == 0 {
            parse_value(15, &mut value, iter);
            bits_taken += 15;

            // need to parse some more packets
            // need to limit the parsing to a certain number of bits....
            let subpacket_sublimit = Some(SubpacketLimit::Bits(value));
            for packet in parse_packets(iter, subpacket_sublimit) {
                bits_taken += packet.get_length();
                subpackets.push(packet);
            }
        } else {
            parse_value(11, &mut value, iter);
            bits_taken += 11;

            // need to limit to a number of packets
            let subpacket_sublimit = Some(SubpacketLimit::Packets(value));
            for packet in parse_packets(iter, subpacket_sublimit) {
                bits_taken += packet.get_length();
                subpackets.push(packet);
            }
        }

        // should use subpacket_limit here to cull bits % 16
        if subpacket_limit.is_none() {
            // we may have only taken 11 bits, so we should move the other 5 0 bits off the end
            let remainder_bits = 16 - (bits_taken % 16);
            for _ in 0..remainder_bits {
                iter.next();
            }
            bits_taken += remainder_bits;
        }
        Self {
            header,
            length: bits_taken,
            length_type_id,
            subpackets,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum SubpacketLimit {
    Bits(u32),
    Packets(u32),
}

fn loop_condition(
    iter: &mut Iter<u8>,
    subpacket_limit: &Option<SubpacketLimit>,
    bits_taken: u32,
    packets_parsed: u32,
) -> bool {
    if iter.as_slice().is_empty() {
        return false;
    }

    match subpacket_limit {
        Some(SubpacketLimit::Bits(num_of_bits)) => bits_taken < *num_of_bits,
        Some(SubpacketLimit::Packets(num_of_packets)) => packets_parsed < *num_of_packets,
        _ => true,
    }
}
fn parse_packets(iter: &mut Iter<u8>, subpacket_limit: Option<SubpacketLimit>) -> Vec<Packet> {
    let mut packets = Vec::new();
    let mut bits_taken = 0;

    // need to loop while:
    //      we have iter to loop
    //      we haven't hit a bit limit
    //      we haven't hit a packet limit
    while loop_condition(
        iter,
        &subpacket_limit,
        bits_taken,
        packets.len().try_into().unwrap(),
    ) {
        // first, get Header
        let header = PacketHeader::from_iter(iter);

        let subpacket_sublimit = match &subpacket_limit {
            None => None,
            Some(SubpacketLimit::Bits(num_of_bits)) => {
                Some(SubpacketLimit::Bits(num_of_bits - bits_taken))
            }
            Some(SubpacketLimit::Packets(num_of_packets)) => {
                let packets_len: u32 = packets.len().try_into().unwrap();
                Some(SubpacketLimit::Packets(num_of_packets - packets_len))
            }
        };
        // switch on the header type to find a parser for the rest of it
        let packet = match header.type_id {
            4 => Some(Packet::LiteralValue(LiteralValuePacket::from_iter(
                iter,
                header,
                subpacket_sublimit,
            ))),
            _ => Some(Packet::Operator(OperatorPacket::from_iter(
                iter,
                header,
                subpacket_sublimit,
            ))),
        };

        if let Some(packet) = packet {
            bits_taken += packet.get_length();
            packets.push(packet);
        }
    }
    // parse the rest, passing in the header
    // get output and put somewhere...
    packets
}

fn parse_value(length: u8, value: &mut u32, iter: &mut Iter<u8>) {
    for _ in 0..length {
        *value <<= 1;
        let val = iter
            .next()
            .expect("Found None while parsing Header struct type_id");
        if *val == 1 {
            *value |= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let test_data = vec!["8A004A801A8002F478"];
        assert_eq!(compute(&test_data).unwrap(), 16);
    }

    #[test]
    fn example2() {
        let test_data = vec!["620080001611562C8802118E34"];
        assert_eq!(compute(&test_data).unwrap(), 12);
    }

    #[test]
    fn example3() {
        let test_data = vec!["C0015000016115A2E0802F182340"];
        assert_eq!(compute(&test_data).unwrap(), 23);
    }

    #[test]
    fn example4() {
        let test_data = vec!["A0016C880162017C3686B18A3D4780"];
        assert_eq!(compute(&test_data).unwrap(), 31);
    }

    #[test]
    fn test_parse_value_1() {
        let test_data = vec![0, 0, 1, 1, 1];
        let mut value = 0;
        parse_value(3, &mut value, &mut test_data.iter());
        assert_eq!(value, 1)
    }

    #[test]
    fn test_parse_value_2() {
        let test_data = vec![0, 0, 1, 1, 1];
        let mut value = 0;
        parse_value(4, &mut value, &mut test_data.iter());
        assert_eq!(value, 3)
    }
}
