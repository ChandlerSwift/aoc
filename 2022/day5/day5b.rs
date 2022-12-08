use std::collections::VecDeque;
use std::fs;

fn process(data: &str) -> String {
    let parts: Vec<&str> = data.split("\n\n").collect();

    let mut stack_lines: Vec<&str> = parts[0].split("\n").collect();
    stack_lines.truncate(stack_lines.len() - 1); // Remove indexing line

    let instructions: Vec<&str> = parts[1].trim().split("\n").collect();

    let mut stacks = Vec::new();
    for _ in 0..(stack_lines[0].len() + 1) / 4 {
        stacks.push(VecDeque::new());
    }

    for line in stack_lines {
        for i in 0..stacks.len() {
            let c = line.chars().nth(i * 4 + 1).unwrap();
            if c != ' ' {
                stacks[i].push_back(c);
            }
        }
    }

    for instruction in instructions {
        let instruction: Vec<&str> = instruction.split(" ").collect();
        let source: usize = instruction[3].parse().unwrap();
        let target: usize = instruction[5].parse().unwrap();
        for i in 0..instruction[1].parse().unwrap() {
            let c = stacks[source-1].pop_front().unwrap();
            stacks[target-1].insert(i, c);
        }
    }
    stacks.iter().map(|s| s.front().unwrap()).collect()
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    println!("{}", process(&data));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let data = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert!(process(data) == "MCD");
    }
}
