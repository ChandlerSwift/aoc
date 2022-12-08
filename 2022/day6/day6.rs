use std::fs;

fn process(data: &str) -> usize {
    let chars: Vec<char> = data.chars().collect();
    for i in 4..chars.len() {
        if chars[i] != chars[i - 1]
            && chars[i] != chars[i - 2]
            && chars[i] != chars[i - 3]
            && chars[i - 1] != chars[i - 2]
            && chars[i - 1] != chars[i - 3]
            && chars[i - 2] != chars[i - 3]
        {
            return i + 1;
        }
    }
    panic!("No valid marker found");
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("{}", process(data));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let data = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];
        for (datum, result) in data {
	    println!("{}: expected {}, got {}", datum, result, process(datum));
            assert!(process(datum) == result);
        }
    }
}
