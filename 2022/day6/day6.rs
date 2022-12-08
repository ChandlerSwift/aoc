use std::fs;

fn process(data: &str, len: usize) -> usize {
    let chars: Vec<char> = data.chars().collect();
    for i in len..chars.len() {
        let mut chars = chars[i-len..i].to_vec(); // TODO: this seems unidiomatic?
        chars.sort();
        chars.dedup();
        if chars.len() == len {
            // no duplicates
            return i;
        }
    }
    panic!("No valid marker found");
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("{}", process(data, 14)); // 4 for part A
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_short() {
        let data = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];
        for (datum, result) in data {
            assert!(process(datum, 4) == result);
        }
    }
    #[test]
    fn test_long() {
        let data = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];
        for (datum, result) in data {
            assert!(process(datum, 14) == result);
        }
    }
}
