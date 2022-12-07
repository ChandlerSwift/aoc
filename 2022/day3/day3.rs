use std::fs;

fn find_duplicated_char(first: &str, second: &str) -> char {
    for char in first.chars() {
        if second.contains(char) {
            return char;
        }
    }
    panic!("no duplicate found");
}

fn priority(c: char) -> u32 {
    match c {
        'A'..='Z' => c as u32 - 64 + 26,
        'a'..='z' => c as u32 - 96,
        _ => panic!("This isn't a letter!"),
    }
}

fn process(data: &str) -> u32 {
    let mut score = 0;
    for line in data.split("\n") {
        let first_half = &line[0..line.len() / 2];
        let second_half = &line[line.len() / 2..line.len()];
        score += priority(find_duplicated_char(first_half, second_half));
    }
    score
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
        let data = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(process(data), 157);
    }
}
