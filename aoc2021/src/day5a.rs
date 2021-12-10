use std::env;
use std::fs;
use std::cmp;

#[allow(dead_code)]
fn format_grid(grid: Vec<Vec<i32>>) -> String {
    let mut ret = String::new();
    for row in grid {
        for cell in row {
            ret += &cell.to_string();
            ret += " ";
        }
        ret += "\n";
    }
    return ret;
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
    let inputs = raw_inputs.trim().split("\n");
    //DEBUG assert_eq!(inputs.len(), 500);
    //DEBUG println!("{}", inputs[0]);
    //DEBUG println!("{}", inputs[499]);
    let mut field: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];
    for input in inputs {
        let coords = input.split(" -> ").collect::<Vec<&str>>();
        let start = coords[0].split(',').collect::<Vec<&str>>();
        let end = coords[1].split(',').collect::<Vec<&str>>();
        let start_x: usize = cmp::min(start[0].parse().unwrap(), end[0].parse().unwrap());
        let start_y: usize = cmp::min(start[1].parse().unwrap(), end[1].parse().unwrap());
        let end_x: usize = cmp::max(start[0].parse().unwrap(), end[0].parse().unwrap());
        let end_y: usize = cmp::max(start[1].parse().unwrap(), end[1].parse().unwrap());
        if start_x != end_x && start_y != end_y {
            //DEBUG println!("Skipping {:?} -> {:?}", start, end);
            continue;
        }
        //DEBUG println!("Drawing line from {},{} to {},{} (inputs {} and {})", start_x, start_y, end_x, end_y, coords[0], coords[1]);
        //DEBUG if start_x == end_x && start_y == end_y {
        //DEBUG     println!("Found matching start/end: {:?} -> {:?}", start, end);
        //DEBUG }
        for x in start_x..=end_x {
            for y in start_y..=end_y {
                field[y][x] += 1;
            }
        }
    }
    /* (0,2)
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [x, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0]
    */

    //DEBUG let mut total = 0;
    //DEBUG for (y, row) in field.iter().enumerate() {
    //DEBUG     for (x, cell) in row.iter().enumerate() {
    //DEBUG         if cell > &1 {
    //DEBUG             if cell > &2 {
    //DEBUG                 println!("Found crossing of value {} at {},{}", cell, x, y);
    //DEBUG             }
    //DEBUG             total += 1;
    //DEBUG         }
    //DEBUG     }
    //DEBUG }
    let mut total = 0;
    for row in field {
        for cell in row {
            if cell > 1 {
                total += 1;
            }
        }
    }
    println!("{}", total);
}

// https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;

    const RAW_MOVES: &str =
        "0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2";

    #[test]
    fn test_apply_puzzle() {
        let inputs = RAW_MOVES.trim().split("\n");
        let mut field: Vec<Vec<i32>> = vec![vec![0; 10]; 10];
        for mut input in inputs {
            input = input.trim();
            let coords = input.split(" -> ").collect::<Vec<&str>>();
            let start = coords[0].split(',').collect::<Vec<&str>>();
            let end = coords[1].split(',').collect::<Vec<&str>>();
            let start_x: usize = cmp::min(start[0].parse().unwrap(), end[0].parse().unwrap());
            let start_y: usize = cmp::min(start[1].parse().unwrap(), end[1].parse().unwrap());
            let end_x: usize = cmp::max(start[0].parse().unwrap(), end[0].parse().unwrap());
            let end_y: usize = cmp::max(start[1].parse().unwrap(), end[1].parse().unwrap());
            if start_x != end_x && start_y != end_y {
                continue;
            }
            for x in start_x..=end_x {
                for y in start_y..=end_y {
                    field[y][x] += 1;
                }
            }
        }

        let mut total = 0;
        for row in &field {
            for cell in row {
                if *cell > 1 {
                    total += 1;
                }
            }
        }
        println!("{}", format_grid(field));
        assert_eq!(total, 5);
    }
}
