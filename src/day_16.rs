use std::cmp::{min, max};

pub fn part_1(input: &str) -> usize {
    let bin_str: String = hex_to_binary(&input);
    let packet: Packet = parse_packet(&bin_str).0;
    sum_versions(&packet)
}

pub fn part_2(input: &str) -> usize {
    let bin_str: String = hex_to_binary(&input);
    let packet: Packet = parse_packet(&bin_str).0;

    evaluate(&packet)
}


#[derive(Debug)]
#[allow(dead_code)]
struct LiteralPacket {
    version: u8,
    type_id: u8,
    value: usize,
}

#[derive(Debug)]
#[allow(dead_code)]
struct OperatorPacket {
    version: u8,
    type_id: u8,
    length_type_id: bool,
    length_field: u16,
    subpackets: Vec<Packet>,
}

#[derive(Debug)]
enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}


fn hex_char_to_binary(hex_char: &char) -> String {
    match hex_char {
        '0' => "0000".to_string(),
        '1' => "0001".to_string(),
        '2' => "0010".to_string(),
        '3' => "0011".to_string(),
        '4' => "0100".to_string(),
        '5' => "0101".to_string(),
        '6' => "0110".to_string(),
        '7' => "0111".to_string(),
        '8' => "1000".to_string(),
        '9' => "1001".to_string(),
        'A' => "1010".to_string(),
        'B' => "1011".to_string(),
        'C' => "1100".to_string(),
        'D' => "1101".to_string(),
        'E' => "1110".to_string(),
        'F' => "1111".to_string(),
        _ => "".to_string(),
    }
}

fn hex_to_binary(hex_str: &str) -> String {
    return hex_str
            .chars()
            .fold(
                "".to_string(), 
                |acc, x| acc + &hex_char_to_binary(&x)
            );
    
}

fn parse_packet(s: &str) -> (Packet, u16) {
    let version: u8 = u8::from_str_radix(&s[0..3], 2).unwrap();
    let type_id: u8 = u8::from_str_radix(&s[3..6], 2).unwrap();
    
    if type_id == 4 {
        // Parse literal
        let mut i: usize = 6;
        let mut string_value: String = String::new();

        while i < s.len() {
            // Parse the group value
            string_value = string_value + &s[i+1..i+5];
            i += 5;

            // Break if the first bit was 0
            if s.chars().nth(i-5).unwrap() == '0' {
                break;
            } 
        }

        let value: usize = usize::from_str_radix(&string_value, 2).unwrap();

        return (Packet::Literal(LiteralPacket{version, type_id, value}), i as u16);

    } else {
        // Parse operator
        let length_type_id: bool = s.chars().nth(6).unwrap() == '1';
        let length_field: u16;
        let mut subpackets: Vec<Packet> = vec![];
        let mut i: u16 = 0;

        if length_type_id {
            // If the length type ID is 1, then the next 11 bits are a number that represents the number of sub-packets immediately contained by this packet.
            length_field = u16::from_str_radix(&s[7..18], 2).unwrap();
            
            let mut n_read: u16 = 0;
            while n_read < length_field {
                let res: (Packet, u16) = parse_packet(&s[(18 + i as usize)..]);
                
                subpackets.push(res.0);
                i += res.1;
                n_read += 1;
            }

            i += 18;
            
        } else {
            // If the length type ID is 0, then the next 15 bits are a number that represents the total length in bits of the sub-packets contained by this packet.
            length_field = u16::from_str_radix(&s[7..22], 2).unwrap();

            while i < length_field {
                let res: (Packet, u16) = parse_packet(&s[(22 + i as usize)..]);
                
                subpackets.push(res.0);
                i += res.1;
            }

            i += 22;
        }

        return (Packet::Operator(OperatorPacket{version, type_id, length_type_id, length_field, subpackets}), i);
    }

}

fn sum_versions(packet: &Packet) -> usize {
    match packet {
        Packet::Literal(p) => p.version as usize,
        Packet::Operator(p) => {
                p
                .subpackets
                .iter()
                .fold(
                    p.version as usize, 
                    |acc, x| acc + sum_versions(x)
                )
        }
    }
}

fn evaluate(packet: &Packet) -> usize {
    match packet {
        Packet::Literal(p) => p.value as usize,
        Packet::Operator(p) => {
            if p.type_id == 0 {
                // 0 - sum
                p.subpackets.iter().fold(0, |acc, x| acc + evaluate(x))
            } else if p.type_id == 1 {
                // 1 - product
                p.subpackets.iter().fold(1, |acc, x| acc * evaluate(x))
            } else if p.type_id == 2 {
                // 2 - minimum
                p.subpackets.iter().fold(usize::MAX, |acc, x| min(acc, evaluate(x)))
            } else if p.type_id == 3 {
                // 3 - maximum
                p.subpackets.iter().fold(0, |acc, x| max(acc, evaluate(x)))
            } else if p.type_id == 5 {
                // 5 - greater than
                (evaluate(&p.subpackets[0]) > evaluate(&p.subpackets[1])) as usize
            } else if p.type_id == 6 {
                // 6 - less than
                (evaluate(&p.subpackets[0]) < evaluate(&p.subpackets[1])) as usize
            } else if p.type_id == 7 {
                // 7 - equal
                (evaluate(&p.subpackets[0]) == evaluate(&p.subpackets[1])) as usize
            } else {
                0
            }
        }
    }
}
