use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        // no additional args; print help text
        eprintln!("Usage: {} infile.txt", args[0]);
        return;
    }

    let filename = &args[1];

    let raw_contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let contents = raw_contents.trim();

    let inputs = contents.split('\n').collect::<Vec<&str>>();
    let inputs_count = inputs.len();
    let digit_count = inputs[0].len();
    let mut digits = vec![0; digit_count];

    for input in inputs {
        for (i, c) in input.chars().enumerate() {
            if c == '1' {
                digits[i] += 1;
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    println!("{:?}", digits);
    for (i, digit) in digits.iter().enumerate() {
        if digit > &(inputs_count/2) {
            gamma += (2 << (digit_count - i - 1))/2;
        } else if digit < &(inputs_count/2) {
            epsilon += (2 << (digit_count - i - 1))/2;
        } else {
            // they're equal -- my reading of the rules doesn't account for this
            panic!();
        }
    }

    println!("Gamma: {}\nEpsilon: {}\nPower Consumption: {}", gamma, epsilon, gamma * epsilon)
}
