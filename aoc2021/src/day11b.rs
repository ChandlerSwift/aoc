use std::env;
use std::fs;
use std::collections::VecDeque;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let mut octopodes = Vec::new();
    for row in input.trim().split("\n") {
        let mut new_row = Vec::new();
        for char in row.trim().chars() {
            new_row.push(char.to_digit(10).unwrap() as u8);
        }
        octopodes.push(new_row);
    }
    return octopodes;
}

fn find_first_round_where_all_flash_simultaneously(mut octopodes: Vec<Vec<u8>>) -> u32 {
    let height = octopodes.len();
    let width = octopodes[0].len();
    let mut round = 0;
    loop {
        let mut number_fired = 0;
        round += 1;
        let mut fired_octopodes: Vec<Vec<bool>> = Vec::new();
        for row in &octopodes {
            let mut fired_row = Vec::new();
            for _ in row {
                fired_row.push(false);
            }
            fired_octopodes.push(fired_row);
        }
    
        // During a single step, the following occurs:
        // First, the energy level of each octopus increases by 1.
        for i in 0..height {
            for j in 0..width {
                octopodes[i][j] += 1;
            }
        }

        // Then, any octopus with an energy level greater than 9 flashes. This
        // increases the energy level of all adjacent octopuses by 1, including
        // octopuses that are diagonally adjacent. If this causes an octopus to have
        // an energy level greater than 9, it also flashes. This process continues
        // as long as new octopuses keep having their energy level increased beyond
        // 9. (An octopus can only flash at most once per step.)
        let mut changed = true;
        while changed {
            changed = false;
            for i in 0..height {
                for j in 0..width {
                    if octopodes[i][j] > 9 && !fired_octopodes[i][j] {
                        number_fired += 1;
                        changed = true;
                        fired_octopodes[i][j] = true;
                        // octopus above
                        if i != 0 {
                            octopodes[i-1][j] += 1;
                        }
                        // above right
                        if i != 0 && j != width - 1 {
                            octopodes[i-1][j+1] += 1;
                        }
                        // octopus to right
                        if j != width - 1 {
                            octopodes[i][j+1] += 1;
                        }
                        // below right
                        if j != width - 1  && i != height - 1 {
                            octopodes[i+1][j+1] += 1;
                        }
                        // octopus below
                        if i != height - 1 {
                            octopodes[i+1][j] += 1;
                        }
                        // below left
                        if i != height - 1 && j != 0 {
                            octopodes[i+1][j-1] += 1;
                        }
                        // octopus to left
                        if j != 0 {
                            octopodes[i][j-1] += 1;
                        }
                        // above left
                        if j != 0 && i != 0 {
                            octopodes[i-1][j-1] += 1;
                        }
                    }
                }
            }
        }

        // Finally, any octopus that flashed during this step has its energy level set to 0, as it used all of its energy to flash.
        for (i, row) in fired_octopodes.iter().enumerate() {
            for (j, fired) in row.iter().enumerate() {
                if *fired {
                    octopodes[i][j] = 0;
                }
            }
        }
        // for row in &octopodes {
        //     println!("{:?}", row);
        // }
        // println!();
        if number_fired == width * height {
            return round;
        }
    }
    panic!("Returned from infinite loop");
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

    println!("{}", find_first_round_where_all_flash_simultaneously(parse_input(raw_inputs.as_str())));
}

// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str ="
    5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526
    ";

    #[test]
    fn test_ten_rounds() {
        assert_eq!(find_first_round_where_all_flash_simultaneously(parse_input(INPUT)), 195);
    }
}
