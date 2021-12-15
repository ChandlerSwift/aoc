use std::env;
use std::fs;

fn find_total_risk_level(inputs: Vec<&str>) -> u32 {
    let mut field: Vec<Vec<u8>> = Vec::new();
    let height = inputs.len();
    let width = inputs[0].len();
    for input in inputs {
        let mut row = Vec::new();
        for c in input.chars() {
            let val = c.to_digit(10);
            match val {
                Some(x) => row.push(x as u8),
                None => {
                    eprintln!("Invalid val '{}'", c);
                    panic!();
                }
            }
        }
        field.push(row);
    }

    let mut total_risk_level = 0;
    for (i, row) in field.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let mut lowest = true;
            // lower than top
            if i != 0 && field[i-1][j] <= *cell {
                lowest = false;
            }
            // bottom
            if i != height - 1 && field[i+1][j] <= *cell {
                lowest = false;
            }
            // left
            if j != 0 && field[i][j-1] <= *cell {
                lowest = false;
            }
            // right
            if j != width - 1 && field[i][j+1] <= *cell {
                lowest = false;
            }
            if lowest {
                total_risk_level += *cell as u32 + 1;
            }
        }
    }
    return total_risk_level;
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
    println!("{}", find_total_risk_level(raw_inputs.trim().split("\n").collect()));
}

// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str ="
2199943210
3987894921
9856789892
8767896789
9899965678
    ";

    #[test]
    fn test_find_total_risk_level() {
        assert_eq!(find_total_risk_level(INPUT.trim().split("\n").collect()), 15);
    }
}
