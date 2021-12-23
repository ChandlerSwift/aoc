use std::env;
use std::fs;
use std::fmt;

struct Packet {
    version: u8, // 3 bits
    type_id: u8, // 3 bits
    subpackets: Vec<Packet>,
    contents: u64,
}

impl Packet {
    fn get_version_sum(&self) -> u64 {
        let mut version_sum = self.version as u64;
        for subpacket in &self.subpackets {
            version_sum += subpacket.get_version_sum();
        }
        version_sum
    }

    fn fmt_with_indentation(&self, f: &mut fmt::Formatter<'_>, indentation_level: u8) -> fmt::Result {
        for _ in 0..indentation_level {
            write!(f, "  ")?;
        }
        write!(f, "version: {}, type: {}, contents: {}, subpackets:", self.version, self.type_id, self.contents)?;
        for subpacket in &self.subpackets {
            subpacket.fmt_with_indentation(f, indentation_level + 1)?;
        }
        write!(f, "")
    }
}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_indentation(f, 0)
    }
}

fn parse_to_binary(s: &str) -> Vec<u8> {
    let mut bin = Vec::new();
    for c in s.chars() {
        bin.append(&mut match c {
            '0' => vec![0, 0, 0, 0],
            '1' => vec![0, 0, 0, 1],
            '2' => vec![0, 0, 1, 0],
            '3' => vec![0, 0, 1, 1],
            '4' => vec![0, 1, 0, 0],
            '5' => vec![0, 1, 0, 1],
            '6' => vec![0, 1, 1, 0],
            '7' => vec![0, 1, 1, 1],
            '8' => vec![1, 0, 0, 0],
            '9' => vec![1, 0, 0, 1],
            'A' => vec![1, 0, 1, 0],
            'B' => vec![1, 0, 1, 1],
            'C' => vec![1, 1, 0, 0],
            'D' => vec![1, 1, 0, 1],
            'E' => vec![1, 1, 1, 0],
            'F' => vec![1, 1, 1, 1],
            _ => panic!("Unknown character"),
        });
    }
    bin
}

fn binary_to_number(bin: &[u8]) -> u64 {
    let mut total = 0;
    for (shift, digit) in bin.iter().rev().enumerate() {
        total += (*digit as u64) << shift;
    }
    total
}

fn parse_packet(packet_binary: Vec<u8>) -> (Packet, u64, Vec<u8>) {
    let (version, rest) = packet_binary.split_at(3);
    let (type_id, mut rest) = rest.split_at(3);
    if binary_to_number(type_id) == 4 {
        let mut contents_binary = Vec::new();
        let mut should_continue = true;
        let mut consumed_bits = 6; // two headers
        while should_continue {
            let (first_five, new_rest) = rest.split_at(5);
            consumed_bits += 5;
            rest = new_rest;
            should_continue = first_five[0] == 1;
            contents_binary.append(&mut first_five[1..5].to_vec())
        }
        return (Packet{
            version: binary_to_number(version) as u8,
            type_id: binary_to_number(type_id) as u8,
            subpackets: Vec::new(),
            contents: binary_to_number(&contents_binary),
        }, consumed_bits, rest.to_vec());
    } else { // Operator packet
        let (length_type_id, rest) = rest.split_at(1);
        if length_type_id[0] == 0 {
            let (subpacket_length_in_bits, rest) = rest.split_at(15);
            let mut rest = rest.to_vec();
            let subpacket_length_in_bits = binary_to_number(subpacket_length_in_bits);
            let mut consumed_bits = 6 + 1 + 15;
            let mut packet = Packet{
                version: binary_to_number(version) as u8,
                type_id: binary_to_number(type_id) as u8,
                subpackets: Vec::new(),
                contents: 0,
            };
            while consumed_bits < subpacket_length_in_bits + 6 + 1 + 15 {
                let (new_packet, additional_bits_consumed, new_rest) = parse_packet(rest);
                packet.subpackets.push(new_packet);
                consumed_bits += additional_bits_consumed;
                rest = new_rest;
            }
            return (packet, consumed_bits, rest.to_vec());
        } else {
            let (number_of_subpackets, rest) = rest.split_at(11);
            let mut rest = rest.to_vec();
            let number_of_subpackets = binary_to_number(number_of_subpackets);
            let mut consumed_bits = 6 + 1 + 11;
            let mut packet = Packet{
                version: binary_to_number(version) as u8,
                type_id: binary_to_number(type_id) as u8,
                subpackets: Vec::new(),
                contents: 0,
            };
            for _ in 0..number_of_subpackets {
                let (new_packet, additional_bits_consumed, new_rest) = parse_packet(rest);
                packet.subpackets.push(new_packet);
                consumed_bits += additional_bits_consumed;
                rest = new_rest;
            }
            return (packet, consumed_bits, rest.to_vec());
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        // no additional args; print help text
        eprintln!("Usage: {} infile.txt", args[0]);
        return;
    }

    let filename = &args[1];
    let raw_input = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let (packet, _packet_len, _rest) = parse_packet(parse_to_binary(raw_input.as_str().trim()));
    
    println!("{}", packet.get_version_sum());
}

// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_to_number() {
        let a = [1, 0, 1, 0, 1, 0];
        assert_eq!(binary_to_number(&a[..]), 42);
    }

    #[test]
    fn test_decode_single_packet() {
        let raw = "D2FE28";
        let (packet, packet_len, rest) = parse_packet(parse_to_binary(raw));
        assert_eq!(packet.version, 6);
        assert_eq!(packet.type_id, 4);
        assert_eq!(packet.contents, 2021);
        assert_eq!(packet.subpackets.len(), 0);
        assert_eq!(packet_len, 21);
        assert_eq!(rest.len(), 3);
    }

    #[test]
    fn test_decode_type_0_with_subpackets() {
        let raw = "38006F45291200";
        let (packet, _packet_len, _rest) = parse_packet(parse_to_binary(raw));
        assert_eq!(packet.version, 1);
        assert_eq!(packet.type_id, 6);
        assert_eq!(packet.contents, 0);
        assert_eq!(packet.subpackets[0].contents, 10);
        assert_eq!(packet.subpackets[1].contents, 20);
    }

    #[test]
    fn test_decode_type_1_with_subpackets() {
        let raw = "EE00D40C823060";
        let (packet, _packet_len, _rest) = parse_packet(parse_to_binary(raw));
        assert_eq!(packet.version, 7);
        assert_eq!(packet.type_id, 3);
        assert_eq!(packet.contents, 0);
        assert_eq!(packet.subpackets[0].contents, 1);
        assert_eq!(packet.subpackets[1].contents, 2);
        assert_eq!(packet.subpackets[2].contents, 3);
    }

    #[test]
    fn test_example_one() {
        let raw = "8A004A801A8002F478";
        let (packet, _packet_len, _rest) = parse_packet(parse_to_binary(raw));
        assert_eq!(packet.version, 4);
        assert_eq!(packet.subpackets[0].version, 1);
        assert_eq!(packet.subpackets[0].subpackets[0].version, 5);
        assert_eq!(packet.subpackets[0].subpackets[0].subpackets[0].version, 6);
        assert_eq!(packet.get_version_sum(), 16);
    }

    #[test]
    fn test_example_two() {
        let raw = "620080001611562C8802118E34";
        let (packet, _packet_len, _rest) = parse_packet(parse_to_binary(raw));
        assert_eq!(packet.version, 3);
        assert_eq!(packet.get_version_sum(), 12);
    }

    #[test]
    fn test_example_three() {
        let raw = "C0015000016115A2E0802F182340";
        let (packet, _packet_len, _rest) = parse_packet(parse_to_binary(raw));
        assert_eq!(packet.get_version_sum(), 23);
    }

    #[test]
    fn test_example_four() {
        let raw = "A0016C880162017C3686B18A3D4780";
        let (packet, _packet_len, _rest) = parse_packet(parse_to_binary(raw));
        assert_eq!(packet.get_version_sum(), 31);
    }
}
