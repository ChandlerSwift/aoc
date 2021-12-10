use std::env;
use std::fs;

fn calculate_fish_count(mut fishes: Vec<i32>, days: u32) -> u32 {
    for _ in 0..days {
        for i in 0..fishes.len() {
            if fishes[i] == 0 {
                fishes[i] = 6;
                fishes.push(8);
            } else {
                fishes[i] -= 1;
            }
        }
    }
    return fishes.len() as u32;
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
    let mut inputs = Vec::new();
    for input in raw_inputs.trim().split(",") {
        inputs.push(input.parse().unwrap());
    }
    println!("{}", calculate_fish_count(inputs, 80));
}

// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str ="3,4,3,1,2";

    #[test]
    fn test_count_fish() {
        let mut inputs = Vec::new();
        for input in INPUT.trim().split(",") {
            inputs.push(input.parse().unwrap());
        }
        assert_eq!(calculate_fish_count(inputs.clone(), 18), 26);
        assert_eq!(calculate_fish_count(inputs.clone(), 80), 5934);
    }
}
