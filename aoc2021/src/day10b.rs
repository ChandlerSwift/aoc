use std::env;
use std::fs;
use std::collections::VecDeque;

fn get_correction_points(input: &str) -> u64 {
    let mut stack = VecDeque::new();
    for c in input.trim().chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push_front(c),
            ')' => match stack.pop_front() {
                Some('(') => continue,
                None => continue,
                _ => return 0, // Something's not right! Ignore this input.
            }
            ']' => match stack.pop_front() {
                Some('[') => continue,
                None => continue,
                _ => return 0,
            }
            '}' => match stack.pop_front() {
                Some('{') => continue,
                None => continue,
                _ => return 0,
            }
            '>' => match stack.pop_front() {
                Some('<') => continue,
                None => continue,
                _ => return 0,
            }
            _ => panic!("Unrecognized character 1")
        }
    }
    let mut points = 0;
    for c in stack.iter() {
        points *= 5;
        points += match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("Unrecognized character 2"),
        }
    }
    return points;
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
    let mut correction_pointses = Vec::new();
    for input in raw_inputs.trim().split("\n").collect::<Vec<&str>>() {
        let points = get_correction_points(input);
        if points > 0 {
            correction_pointses.push(points);
        }
    }
    correction_pointses.sort();
    println!("{}", correction_pointses[correction_pointses.len()/2]);
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
    fn find_total_correction_level() {
        let mut correction_total = 0;
        for input in INPUT.trim().split("\n").collect::<Vec<&str>>() {
            let points = get_correction_points(input);
            correction_total += points;
            println!("Got {} for input {}", points, input)
        }
        assert_eq!(correction_total, 288957+5566+1480781+995444+294);
    }

    #[test]
    fn find_middle_correction_level() {
        let mut correction_pointses = Vec::new();
        for input in INPUT.trim().split("\n").collect::<Vec<&str>>() {
            let points = get_correction_points(input);
            if points > 0 {
                correction_pointses.push(points);
            }
            println!("Got {} for input {}", points, input)
        }
        correction_pointses.sort();
        assert_eq!(correction_pointses[correction_pointses.len()/2], 288957);
    }
}
