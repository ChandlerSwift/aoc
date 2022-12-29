use std::cmp::{min, Ordering};
use std::fs;

#[derive(Debug, PartialEq, Eq, Clone)]
enum List {
    Integer(u8),
    List(Vec<List>),
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // If both values are integers, the lower integer should
            // come first.
            (List::Integer(s), List::Integer(o)) => s.cmp(o),
            // If both values are lists, compare the first value of
            // each list, then the second value, and so on. If the
            // left list runs out of items first, the inputs are in
            // the right order.
            (List::List(s), List::List(o)) => {
                for i in 0..min(s.len(), o.len()) {
                    if s[i] != o[i] {
                        return s[i].cmp(&o[i]);
                    }
                }
                s.len().cmp(&o.len())
            }
            // If exactly one value is an integer, convert the integer
            // to a list which contains that integer as its only
            // value, then retry the comparison.
            (List::Integer(s), List::List(o)) => {
                List::List(vec![List::Integer(*s)]).cmp(&List::List(o.to_vec()))
            }
            (List::List(s), List::Integer(o)) => {
                List::List(s.to_vec()).cmp(&List::List(vec![List::Integer(*o)]))
            }
        }
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(data: &str) -> List {
    let mut data = &data[1..data.len() - 1]; // Strip square brackets
    let mut packet = Vec::new();
    while data.len() > 0 {
        match data.chars().next().unwrap() {
            '[' => {
                // next element is a list
                let mut list_str = String::new();
                let mut depth = 0;
                loop {
                    list_str.push(data.chars().next().unwrap());
                    match data.chars().next().unwrap() {
                        '[' => depth += 1,
                        ']' => depth -= 1,
                        _ => (),
                    };
                    data = &data[1..];
                    if depth == 0 {
                        break;
                    }
                }
                packet.push(parse(&list_str));
            }
            ',' => data = &data[1..],
            _ => {
                // next element is an integer
                let mut num = String::new();
                while data.len() > 0 {
                    match data.chars().next().unwrap() {
                        ',' => break,
                        c => num.push(c), // TODO
                    };
                    data = &data[1..];
                }
                packet.push(List::Integer(num.parse().unwrap()));
            }
        }
    }
    List::List(packet)
}

fn process_part1(data: &str) -> usize {
    let mut sum = 0;
    for (index, packet_pair) in data.split("\n\n").enumerate() {
        let packets: Vec<&str> = packet_pair.split("\n").collect();
        let zeroth = parse(packets[0]);
        let first = parse(packets[1]);
        match zeroth.cmp(&first) {
            Ordering::Less => sum += index + 1,
            Ordering::Greater => (),
            Ordering::Equal => panic!("{:?} and {:?} are equal", zeroth, first),
        }
    }
    sum
}

fn process_part2(data: &str) -> usize {
    let mut packets: Vec<List> = data
        .split("\n")
        .filter(|p| p != &"")
        .map(|p| parse(p))
        .collect();
    let zeroth_divider = parse("[[2]]");
    let first_divider = parse("[[6]]");
    packets.push(zeroth_divider.clone());
    packets.push(first_divider.clone());
    packets.sort();
    let mut product = 1;
    for (index, packet) in packets.iter().enumerate() {
        if *packet == zeroth_divider || *packet == first_divider {
            product *= index + 1;
        }
    }
    product
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("{}", process_part1(data));
    println!("{}", process_part2(data));
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_flat() {
        let data = "[1,1,3,1,1]\n[1,1,5,1,1]";
        assert_eq!(process_part1(data), 1);
    }

    #[test]
    fn test_part1() {
        assert_eq!(process_part1(DATA), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(process_part2(DATA), 140);
    }
}
