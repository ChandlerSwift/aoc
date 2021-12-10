use std::env;
use std::fs;

fn calculate_optimal_position(crabs: Vec<u32>) -> (u32, u32) {
    // Efficiency? What?
    let max_position = crabs.iter().max().unwrap();
    let mut min_fuel_used = u32::MAX;
    let mut best_position = 0;
    for position in 0..=*max_position {
        let mut fuel_used = 0;
        for crab in &crabs {
            fuel_used += (position as i32 - *crab as i32).abs() as u32;
        }
        if fuel_used < min_fuel_used {
            min_fuel_used = fuel_used;
            best_position = position;
        }
    }
    return (best_position, min_fuel_used);
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
    println!("{}", calculate_optimal_position(inputs).1);
}

// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str ="16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_find_best_crab_position() {
        let mut inputs = Vec::new();
        for input in INPUT.trim().split(",") {
            inputs.push(input.parse().unwrap());
        }
        assert_eq!(calculate_optimal_position(inputs), (2, 37));
    }
}
