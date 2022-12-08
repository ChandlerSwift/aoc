use std::fs;

fn process(data: &str) -> u32 {
    let mut contains_count = 0;
    for line in data.split("\n") {
        let ranges: Vec<&str> = line.split(",").collect();
        let first: Vec<&str> = ranges[0].split("-").collect();
        let second: Vec<&str> = ranges[1].split("-").collect();
        let first_start: u32 = first[0].parse().unwrap();
        let first_end: u32 = first[1].parse().unwrap();
        let second_start: u32 = second[0].parse().unwrap();
        let second_end: u32 = second[1].parse().unwrap();
        if first_start <= second_end && second_start <= first_end {
            contains_count += 1;
        }
    }
    contains_count
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Could not read input.txt");
    let data = data.trim();

    println!("{}", process(data));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let data = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(process(data), 4);
    }
}
