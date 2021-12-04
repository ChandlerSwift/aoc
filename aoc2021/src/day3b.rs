use std::env;
use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum Rating {
    Oxygen,
    CarbonDioxide,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        // no additional args; print help text
        eprintln!("Usage: {} infile.txt", args[0]);
        return;
    }

    let filename = &args[1];

    // ??? If I don't add the intermediate `let` binding, this errors:
    //
    // temporary value dropped while borrowed
    // creates a temporary which is freed while still in use
    // note: consider using a `let` binding to create a longer lived valuerustc(E0716)
    // day3b.rs(21, 18): creates a temporary which is freed while still in use
    // day3b.rs(21, 136): temporary value is freed at the end of this statement
    // day3b.rs(23, 47): borrow later used here
    //
    // let inputs = fs::read_to_string(filename).expect("Something went wrong reading the file").trim().split('\n').collect::<Vec<&str>>();
    let raw_inputs = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let inputs = raw_inputs.trim().split('\n').collect::<Vec<&str>>();

    let oxygen_generator_binary = find_rating(inputs.clone(), Rating::Oxygen);
    let oxygen_generator_rating = isize::from_str_radix(oxygen_generator_binary, 2).unwrap();
    let co2_scrubber_binary = find_rating(inputs, Rating::CarbonDioxide);
    let co2_scrubber_rating = isize::from_str_radix(co2_scrubber_binary, 2).unwrap();

    println!(
        "Oxygen: {} ({})\nCO₂: {} ({})\nLife Support: {}",
        oxygen_generator_rating,
        oxygen_generator_binary,
        co2_scrubber_rating,
        co2_scrubber_binary,
        oxygen_generator_rating * co2_scrubber_rating
    )
}

fn find_rating(mut inputs: Vec<&str>, mode: Rating) -> &str {
    let digit_count = inputs[0].len();
    for digit in 0..digit_count {
        let mut ones_in_digit_th_place = 0;
        for input in inputs.iter() {
            // Adding iter() makes the borrow checker happy I guess?
            if input.chars().nth(digit).unwrap() == '1' {
                ones_in_digit_th_place += 1;
            }
        }
        let target_digit;
        if mode == Rating::Oxygen {
            if ones_in_digit_th_place * 2 >= inputs.len() {
                target_digit = '1';
            } else {
                target_digit = '0';
            }
        } else { // mode == Rating::CarbonDioxide
            if ones_in_digit_th_place * 2 >= inputs.len() {
                target_digit = '0';
            } else {
                target_digit = '1';
            }
        }

        inputs = inputs
            .iter()
            .filter(|x| x.chars().nth(digit).unwrap() == target_digit)
            .cloned() // ??? https://stackoverflow.com/questions/54273751/rust-and-vec-iterator-how-to-filter
            .collect();

        // DEBUG: println!("Remaining at {}: {} ({:?})", digit, inputs.len(), inputs);
        if inputs.len() == 1 {
            break;
        }
    }
    assert_eq!(inputs.len(), 1); // If we have more than one input remaining, we've messed up
    return inputs[0];
}

// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;

    const RAW_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_oxygen_generator() {
        let input = RAW_INPUT.split('\n').collect::<Vec<&str>>();
        assert_eq!(find_rating(input, Rating::Oxygen), "10111");
    }

    #[test]
    fn test_co2_scrubber() {
        println!("{}", RAW_INPUT);
        let input = RAW_INPUT.split('\n').collect::<Vec<&str>>();
        assert_eq!(find_rating(input, Rating::CarbonDioxide), "01010");
    }
}
