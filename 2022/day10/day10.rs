use std::collections::HashMap;
use std::fs;

fn process(data: &str) -> i32 {
    let mut current_cycle = 1;
    let mut accumulator = 1;
    let mut value_at_cycle = HashMap::new();
    for line in data.split("\n") {
        let instruction: Vec<&str> = line.split(" ").collect();
        match instruction[0] {
            "noop" => {
                value_at_cycle.insert(current_cycle, accumulator);
                current_cycle += 1;
            }
            "addx" => {
                value_at_cycle.insert(current_cycle, accumulator);
                current_cycle += 1;
                value_at_cycle.insert(current_cycle, accumulator);
                current_cycle += 1;
                accumulator += instruction[1].parse::<i32>().unwrap();
            }
            _ => panic!("Unknown instruction {}", instruction[0]),
        }
    }
    let mut total_signal_strength = 0;
    for i in vec![20, 60, 100, 140, 180, 220] {
        total_signal_strength += i * value_at_cycle[&i];
    }
    total_signal_strength
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();
    let data = data.trim();
    println!("{}", process(data));
}

#[cfg(test)]
mod test {
    use super::*;

    static DATA: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test() {
        assert!(process(DATA) == 13140);
    }
}
