use std::env;
use std::fs;

fn calculate_fish_count(fishes: Vec<i32>, days: u32) -> u64 {
    let mut fish_by_day = vec![0 as u64; 9];
    for fish in fishes {
        fish_by_day[fish as usize] += 1;
    }
    for _ in 0..days {
        let fish_ready_to_spawn_another = fish_by_day[0];
        fish_by_day[0] = fish_by_day[1];
        fish_by_day[1] = fish_by_day[2];
        fish_by_day[2] = fish_by_day[3];
        fish_by_day[3] = fish_by_day[4];
        fish_by_day[4] = fish_by_day[5];
        fish_by_day[5] = fish_by_day[6];
        fish_by_day[6] = fish_by_day[7];
        fish_by_day[7] = fish_by_day[8];
        fish_by_day[6] += fish_ready_to_spawn_another;
        fish_by_day[8] = fish_ready_to_spawn_another;
    }
    return fish_by_day.iter().sum();
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
    println!("{}", calculate_fish_count(inputs, 256));
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
        assert_eq!(calculate_fish_count(inputs.clone(), 256), 26984457539);
    }
}
