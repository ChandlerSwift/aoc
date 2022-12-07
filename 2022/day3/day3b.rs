use std::fs;

fn find_duplicated_char(first: &str, second: &str, third: &str) -> char {
    for char in first.chars() {
        if second.contains(char) && third.contains(char) {
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
    let lines: Vec<&str> = data.split("\n").collect();
    let mut i = 0;
    while i < lines.len() {
        score += priority(find_duplicated_char(lines[i], lines[i + 1], lines[i + 2]));
        i += 3;
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
        assert_eq!(process(data), 70);
    }
}
