use std::env;
use std::fs;
use std::collections::VecDeque;

fn get_corruption_points(input: &str) -> u32 {
    let mut stack = VecDeque::new();
    for c in input.trim().chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push_front(c),
            ')' => match stack.pop_front() {
                Some('(') => continue,
                None => continue,
                _ => return 3,
            }
            ']' => match stack.pop_front() {
                Some('[') => continue,
                None => continue,
                _ => return 57,
            }
            '}' => match stack.pop_front() {
                Some('{') => continue,
                None => continue,
                _ => return 1197,
            }
            '>' => match stack.pop_front() {
                Some('<') => continue,
                None => continue,
                _ => return 25137,
            }
            _ => panic!("Unrecognized character")
        }
    }
    return 0;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        // no additional args; print help text
        eprintln!("Usage: {} infile.txt", args[0]);
        return;
    }

    let filename = &args[1];
    let raw_inputs = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut corruption_total = 0;
    for input in raw_inputs.trim().split("\n").collect::<Vec<&str>>() {
        corruption_total += get_corruption_points(input);
    }
    println!("{}", corruption_total);
}

// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str ="
    [({(<(())[]>[[{[]{<()<>>
    [(()[<>])]({[<{<<[]>>(
    {([(<{}[<>[]}>{[]{[(<()>
    (((({<>}<{<{<>}{[]{[]{}
    [[<[([]))<([[{}[[()]]]
    [{[{({}]{}}([{[{{{}}([]
    {<[[]]>}<{[{[{[]{()[[[]
    [<(<(<(<{}))><([]([]()
    <{([([[(<>()){}]>(<<{{
    <{([{{}}[<[[[<>{}]]]>[]]
    ";

    #[test]
    fn test_find_total_risk_level() {
        let mut corruption_total = 0;
        for input in INPUT.trim().split("\n").collect::<Vec<&str>>() {
            let points = get_corruption_points(input);
            corruption_total += points;
            println!("Got {} for input {}", points, input)
        }
        assert_eq!(corruption_total, 26397);
    }
}
